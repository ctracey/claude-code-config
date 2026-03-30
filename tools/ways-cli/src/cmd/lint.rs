use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn run(path: Option<String>, schema: bool, check: bool) -> Result<()> {
    let ways_dir = home_dir().join(".claude/hooks/ways");
    let schema_path = ways_dir.join("frontmatter-schema.yaml");

    if schema {
        let content = std::fs::read_to_string(&schema_path)
            .with_context(|| format!("reading {}", schema_path.display()))?;
        print!("{content}");
        return Ok(());
    }

    let schema_data = load_schema(&schema_path)?;

    eprintln!();
    eprintln!("Way Frontmatter Lint");
    eprintln!("Schema: {}", schema_path.display());
    eprintln!();

    let mut errors = 0u32;
    let mut warnings = 0u32;

    let is_targeted = path.is_some();
    let scan_dir = path.map(PathBuf::from).unwrap_or(ways_dir.clone());
    let file_count = scan_and_lint(&scan_dir, &ways_dir, &schema_data, &mut errors, &mut warnings)?;

    // Provenance sidecar validation
    lint_provenance_sidecars(&scan_dir, &ways_dir, &mut errors)?;

    let label = if is_targeted { "Target" } else { "Global" };
    eprintln!("{label}: scanned {file_count} files");
    eprintln!();
    eprintln!("Summary: {errors} errors, {warnings} warnings");

    if check && errors > 0 {
        std::process::exit(1);
    }
    Ok(())
}

// ── Schema ──────────────────────────────────────────────────────

struct Schema {
    way_fields: HashSet<String>,
    check_fields: HashSet<String>,
    when_subfields: HashSet<String>,
    valid_scopes: Vec<String>,
    valid_macros: Vec<String>,
    valid_triggers: Vec<String>,
}

fn load_schema(path: &Path) -> Result<Schema> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading schema {}", path.display()))?;
    let doc: serde_yaml::Value = serde_yaml::from_str(&content)?;

    let way_fields = extract_fields(&doc, "way");
    let check_fields = extract_fields(&doc, "check");
    let when_subfields = extract_when_subfields(&doc);
    let valid_scopes = extract_enum_values(&doc, "way", "scope");
    let valid_macros = extract_enum_values(&doc, "way", "macro");
    let valid_triggers = extract_enum_values(&doc, "way", "trigger");

    Ok(Schema {
        way_fields,
        check_fields,
        when_subfields,
        valid_scopes,
        valid_macros,
        valid_triggers,
    })
}

fn extract_fields(doc: &serde_yaml::Value, type_name: &str) -> HashSet<String> {
    let mut fields = HashSet::new();
    if let Some(type_block) = doc.get(type_name).and_then(|v| v.as_mapping()) {
        for (_category, category_block) in type_block {
            if let Some(mapping) = category_block.as_mapping() {
                for (field_name, _) in mapping {
                    if let Some(name) = field_name.as_str() {
                        fields.insert(name.to_string());
                    }
                }
            }
        }
    }
    fields
}

fn extract_when_subfields(doc: &serde_yaml::Value) -> HashSet<String> {
    let mut fields = HashSet::new();
    if let Some(subfields) = doc
        .get("way")
        .and_then(|v| v.get("preconditions"))
        .and_then(|v| v.get("when"))
        .and_then(|v| v.get("subfields"))
        .and_then(|v| v.as_mapping())
    {
        for (name, _) in subfields {
            if let Some(n) = name.as_str() {
                fields.insert(n.to_string());
            }
        }
    }
    fields
}

fn extract_enum_values(doc: &serde_yaml::Value, type_name: &str, field_name: &str) -> Vec<String> {
    let type_block = match doc.get(type_name).and_then(|v| v.as_mapping()) {
        Some(m) => m,
        None => return Vec::new(),
    };

    for (_category, category_block) in type_block {
        if let Some(field) = category_block.get(field_name) {
            if let Some(values) = field.get("values").and_then(|v| v.as_sequence()) {
                return values
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }
    }
    Vec::new()
}

// ── Scanning ────────────────────────────────────────────────────

fn scan_and_lint(
    dir: &Path,
    ways_dir: &Path,
    schema: &Schema,
    errors: &mut u32,
    warnings: &mut u32,
) -> Result<usize> {
    let mut files: Vec<(PathBuf, bool)> = Vec::new(); // (path, is_check)

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        // Check if it has frontmatter
        let first_line = match std::fs::read_to_string(path) {
            Ok(c) => c.lines().next().unwrap_or("").to_string(),
            Err(_) => continue,
        };
        if first_line != "---" {
            continue;
        }

        let is_check = path
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n.contains(".check."));

        files.push((path.to_path_buf(), is_check));
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));

    for (path, is_check) in &files {
        lint_file(path, *is_check, ways_dir, schema, errors, warnings)?;
    }

    Ok(files.len())
}

// ── Per-file linting ────────────────────────────────────────────

fn lint_file(
    path: &Path,
    is_check: bool,
    ways_dir: &Path,
    schema: &Schema,
    errors: &mut u32,
    warnings: &mut u32,
) -> Result<()> {
    let content = std::fs::read_to_string(path)?;
    let relpath = path.strip_prefix(ways_dir).unwrap_or(path);
    let rel = relpath.display();

    let fm_str = match extract_frontmatter_raw(&content) {
        Some(s) => s,
        None => {
            eprintln!("  ERROR: {rel} — no YAML frontmatter");
            *errors += 1;
            return Ok(());
        }
    };

    let valid_fields = if is_check {
        &schema.check_fields
    } else {
        &schema.way_fields
    };

    // Multi-line YAML values (> or |) — trigger pipeline can't parse them
    for line in fm_str.lines() {
        if let Some(field) = line.strip_suffix(": >").or_else(|| line.strip_suffix(": |")) {
            let field = field.trim();
            eprintln!("  ERROR: {rel} — '{field}' uses multi-line YAML (> or |) which the trigger pipeline cannot parse. Use a single line.");
            *errors += 1;
        }
    }

    // Unknown fields
    for line in fm_str.lines() {
        if let Some(field) = extract_field_name(line) {
            if !valid_fields.contains(field) {
                eprintln!("  UNKNOWN: {rel} — unknown field '{field}'");
                *warnings += 1;
            }
        }
    }

    // Description/vocabulary conditional pairing
    let has_desc = has_field(&fm_str, "description");
    let has_vocab = has_field(&fm_str, "vocabulary");
    if has_desc && !has_vocab {
        eprintln!("  WARNING: {rel} — description without vocabulary (semantic matching incomplete)");
        *warnings += 1;
    }
    if has_vocab && !has_desc {
        eprintln!("  WARNING: {rel} — vocabulary without description (semantic matching incomplete)");
        *warnings += 1;
    }

    // Threshold is numeric
    if let Some(val) = get_field_value(&fm_str, "threshold") {
        if val.parse::<f64>().is_err() {
            eprintln!("  ERROR: {rel} — threshold '{val}' is not numeric");
            *errors += 1;
        }
    }

    // Scope enum
    if let Some(val) = get_field_value(&fm_str, "scope") {
        for part in val.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if !schema.valid_scopes.iter().any(|v| v == part) {
                eprintln!(
                    "  ERROR: {rel} — invalid scope '{part}' (valid: {})",
                    schema.valid_scopes.join(", ")
                );
                *errors += 1;
            }
        }
    }

    // Macro enum
    if let Some(val) = get_field_value(&fm_str, "macro") {
        if !schema.valid_macros.iter().any(|v| v == &val) {
            eprintln!(
                "  ERROR: {rel} — invalid macro '{val}' (valid: {})",
                schema.valid_macros.join(", ")
            );
            *errors += 1;
        }
    }

    // Trigger enum
    if let Some(val) = get_field_value(&fm_str, "trigger") {
        if !schema.valid_triggers.iter().any(|v| v == &val) {
            eprintln!(
                "  ERROR: {rel} — invalid trigger '{val}' (valid: {})",
                schema.valid_triggers.join(", ")
            );
            *errors += 1;
        }
    }

    // when: subfields
    let when_block = extract_when_block(&fm_str);
    for line in when_block.lines() {
        if let Some(field) = extract_indented_field_name(line) {
            if !schema.when_subfields.contains(field) {
                eprintln!("  UNKNOWN: {rel} — unknown when: sub-field '{field}'");
                *warnings += 1;
            }
        }
    }

    // when.project path existence
    for line in when_block.lines() {
        if let Some(val) = line.strip_prefix("  project:").or_else(|| line.strip_prefix("project:")) {
            let path_str = val.trim();
            let expanded = path_str.replace("~", &home_dir().display().to_string());
            if !Path::new(&expanded).is_dir() {
                eprintln!("  WARNING: {rel} — when.project path '{path_str}' does not exist");
                *warnings += 1;
            }
        }
    }

    // check file: verify anchor and check sections
    if is_check {
        if !content.contains("\n## anchor") && !content.starts_with("## anchor") {
            eprintln!("  ERROR: {rel} — check file missing '## anchor' section");
            *errors += 1;
        }
        if !content.contains("\n## check") && !content.starts_with("## check") {
            eprintln!("  ERROR: {rel} — check file missing '## check' section");
            *errors += 1;
        }
    }

    Ok(())
}

// ── Provenance sidecar validation ───────────────────────────────

fn lint_provenance_sidecars(dir: &Path, ways_dir: &Path, errors: &mut u32) -> Result<()> {
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file()
            || path.file_name().and_then(|n| n.to_str()) != Some("provenance.yaml")
        {
            continue;
        }

        let relpath = path.strip_prefix(ways_dir).unwrap_or(path);
        let content = std::fs::read_to_string(path)?;
        let parsed: Result<serde_yaml::Value, _> = serde_yaml::from_str(&content);

        match parsed {
            Ok(val) => {
                if let Some(map) = val.as_mapping() {
                    if map.get("policy").is_none() && map.get("controls").is_none() {
                        eprintln!(
                            "  ERROR: {} — invalid provenance sidecar (needs policy or controls)",
                            relpath.display()
                        );
                        *errors += 1;
                    }
                } else {
                    eprintln!(
                        "  ERROR: {} — provenance sidecar is not a YAML mapping",
                        relpath.display()
                    );
                    *errors += 1;
                }
            }
            Err(_) => {
                eprintln!(
                    "  ERROR: {} — provenance sidecar has invalid YAML",
                    relpath.display()
                );
                *errors += 1;
            }
        }
    }
    Ok(())
}

// ── Helpers ─────────────────────────────────────────────────────

fn extract_frontmatter_raw(content: &str) -> Option<String> {
    let mut lines = content.lines();
    if lines.next()? != "---" {
        return None;
    }
    let mut fm_lines = Vec::new();
    for line in lines {
        if line == "---" {
            return Some(fm_lines.join("\n"));
        }
        fm_lines.push(line);
    }
    None
}

fn extract_field_name(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    // Only top-level fields (no leading whitespace for frontmatter fields)
    if trimmed.len() != line.len() && !line.starts_with("  ") {
        return None;
    }
    // Top-level: starts with a-z
    if line.starts_with(|c: char| c.is_ascii_lowercase()) {
        let colon_pos = line.find(':')?;
        let field = &line[..colon_pos];
        if field.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Some(field);
        }
    }
    None
}

fn extract_indented_field_name(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return None;
    }
    let colon_pos = trimmed.find(':')?;
    let field = &trimmed[..colon_pos];
    if field.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
        Some(field)
    } else {
        None
    }
}

fn has_field(fm: &str, name: &str) -> bool {
    let prefix = format!("{name}:");
    fm.lines().any(|l| l.starts_with(&prefix))
}

fn get_field_value<'a>(fm: &'a str, name: &str) -> Option<String> {
    let prefix = format!("{name}:");
    for line in fm.lines() {
        if let Some(rest) = line.strip_prefix(&prefix) {
            let val = rest.trim().to_string();
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    None
}

fn extract_when_block(fm: &str) -> String {
    let mut lines = Vec::new();
    let mut in_when = false;
    for line in fm.lines() {
        if line == "when:" {
            in_when = true;
            continue;
        }
        if in_when {
            if line.starts_with("  ") {
                lines.push(line);
            } else if !line.is_empty() {
                break;
            }
        }
    }
    lines.join("\n")
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

//! Display ways, checks, and core guidance — session-aware, idempotent.
//!
//! Replaces: show-way.sh, show-check.sh, show-core.sh

use anyhow::Result;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::session;

// ── ways show way ───────────────────────────────────────────────

pub fn way(id: &str, session_id: &str, trigger: &str) -> Result<()> {
    let project_dir = std::env::var("CLAUDE_PROJECT_DIR")
        .unwrap_or_else(|_| std::env::var("PWD").unwrap_or_else(|_| ".".to_string()));

    // Domain disable check
    let domain = id.split('/').next().unwrap_or(id);
    if session::domain_disabled(domain) {
        return Ok(());
    }

    // Scope check
    let scope = session::detect_scope(session_id);
    let (way_file, is_project_local) = match session::resolve_way_file(id, &project_dir) {
        Some(r) => r,
        None => return Ok(()), // way not found
    };

    // Read frontmatter for scope field
    let content = std::fs::read_to_string(&way_file)?;
    let scope_field = extract_field(&content, "scope").unwrap_or_default();
    if !session::scope_matches(&scope_field, &scope) {
        return Ok(());
    }

    // Session marker check + token-gated re-disclosure (ADR-104)
    let is_redisclosure;
    if session::way_is_shown(id, session_id) {
        match session::token_distance_exceeded(id, session_id) {
            Some(_distance) => {
                is_redisclosure = true;
            }
            None => return Ok(()), // still warm, no-op
        }
    } else {
        is_redisclosure = false;
    }

    // Macro handling
    let macro_pos = extract_field(&content, "macro");
    let way_dir = way_file.parent().unwrap_or(Path::new("."));
    let macro_file = way_dir.join("macro.sh");
    let macro_out = if macro_pos.is_some() && macro_file.is_file() {
        // Security: skip project-local macros unless trusted
        if is_project_local && !is_project_trusted(&project_dir) {
            Some(format!(
                "**Note**: Project-local macro skipped (add {} to ~/.claude/trusted-project-macros to enable)",
                project_dir
            ))
        } else {
            run_macro(&macro_file)
        }
    } else {
        None
    };

    // Output content
    if macro_pos.as_deref() == Some("prepend") {
        if let Some(ref out) = macro_out {
            println!("{out}\n");
        }
    }

    // Strip frontmatter, output body
    print_body(&content);

    if macro_pos.as_deref() == Some("append") {
        if let Some(ref out) = macro_out {
            println!("\n{out}");
        }
    }

    // Stamp markers
    let token_pos = session::get_token_position(session_id);
    session::stamp_way_marker(id, session_id, token_pos);
    session::stamp_way_tokens(id, session_id, token_pos);

    let epoch = session::get_epoch(session_id);
    session::stamp_way_epoch(id, session_id, epoch);

    // Tree disclosure tracking
    let (tree_depth, parent_id, parent_epoch, epoch_from_parent) =
        compute_tree_metrics(id, session_id);

    let (sibling_total, sibling_fired) = count_siblings(id, &project_dir, session_id);

    // Metrics JSONL
    session::append_metric(
        session_id,
        &json!({
            "way": id,
            "parent": parent_id.as_deref().unwrap_or("none"),
            "depth": tree_depth,
            "epoch": epoch,
            "parent_epoch": parent_epoch,
            "epoch_distance": epoch_from_parent,
            "sibling_total": sibling_total,
            "sibling_fired": sibling_fired,
            "trigger": trigger,
        }),
    );

    // Event logging
    let mut log_fields: Vec<(&str, String)> = vec![
        ("event", if is_redisclosure { "way_redisclosed" } else { "way_fired" }.to_string()),
        ("way", id.to_string()),
        ("domain", domain.to_string()),
        ("trigger", trigger.to_string()),
        ("scope", scope),
        ("project", project_dir),
        ("session", session_id.to_string()),
    ];
    if let Some(ref p) = parent_id {
        log_fields.push(("parent", p.clone()));
        log_fields.push(("tree_depth", tree_depth.to_string()));
        if let Some(dist) = epoch_from_parent {
            log_fields.push(("epoch_distance", dist.to_string()));
        }
    }
    let team = session::detect_team(session_id);
    if let Some(t) = team {
        log_fields.push(("team", t));
    }
    let refs: Vec<(&str, &str)> = log_fields.iter().map(|(k, v)| (*k, v.as_str())).collect();
    session::log_event(&refs);

    Ok(())
}

// ── ways show check ─────────────────────────────────────────────

pub fn check(id: &str, session_id: &str, trigger: &str, match_score: f64) -> Result<()> {
    let project_dir = std::env::var("CLAUDE_PROJECT_DIR")
        .unwrap_or_else(|_| std::env::var("PWD").unwrap_or_else(|_| ".".to_string()));

    // Domain disable
    let domain = id.split('/').next().unwrap_or(id);
    if session::domain_disabled(domain) {
        return Ok(());
    }

    // Scope check
    let scope = session::detect_scope(session_id);

    let (check_file, _is_project_local) = match session::resolve_check_file(id, &project_dir) {
        Some(r) => r,
        None => return Ok(()),
    };

    let check_content = std::fs::read_to_string(&check_file)?;
    let scope_field = extract_field(&check_content, "scope").unwrap_or_default();
    if !scope_field.is_empty() && !session::scope_matches(&scope_field, &scope) {
        return Ok(());
    }

    // Epoch distance
    let epoch = session::get_epoch(session_id);
    let way_has_fired = session::way_is_shown(id, session_id);
    let epoch_distance = if way_has_fired {
        session::epoch_distance(id, session_id).min(30) // cap
    } else {
        30 // way hasn't fired — max distance
    };

    // Fire count
    let fire_count = session::get_check_fires(id, session_id);

    // Scoring curve: effective_score = match_score × distance_factor × decay_factor
    let distance_factor = ((epoch_distance as f64) + 1.0).ln() + 1.0;
    let decay_factor = 1.0 / (fire_count as f64 + 1.0);
    let effective_score = match_score * distance_factor * decay_factor;

    // Threshold
    let threshold: f64 = extract_field(&check_content, "threshold")
        .and_then(|s| s.parse().ok())
        .unwrap_or(2.0);

    if effective_score < threshold {
        return Ok(());
    }

    // Output
    // If parent way hasn't fired, pull it in alongside the check
    if !way_has_fired {
        // Call ourselves recursively for the parent way
        let _ = way(id, session_id, "check-pull");
        println!();
    }

    // Include anchor section when epoch distance >= 5
    let include_anchor = epoch_distance >= 5;
    print_check_sections(&check_content, include_anchor);

    // Bump fire count
    session::bump_check_fires(id, session_id);

    // Log
    let anchored = if include_anchor { "true" } else { "false" };
    let way_epoch = session::get_way_epoch(id, session_id);
    session::log_event(&[
        ("event", "check_fired"),
        ("check", id),
        ("domain", domain),
        ("trigger", trigger),
        ("epoch", &epoch.to_string()),
        ("way_epoch", &way_epoch.to_string()),
        ("distance", &epoch_distance.to_string()),
        ("fire_count", &(fire_count + 1).to_string()),
        ("match_score", &format!("{match_score:.2}")),
        ("effective_score", &format!("{effective_score:.2}")),
        ("anchored", anchored),
        ("scope", &scope),
        ("project", &project_dir),
        ("session", session_id),
    ]);

    Ok(())
}

// ── ways show core ──────────────────────────────────────────────

pub fn core(session_id: &str) -> Result<()> {
    let ways_dir = home_dir().join(".claude/hooks/ways");

    // Run the macro for the dynamic ways table
    let macro_file = ways_dir.join("macro.sh");
    if macro_file.is_file() {
        if let Some(out) = run_macro(&macro_file) {
            println!("{out}");
        }
    }

    // Output core.md body
    let core_file = ways_dir.join("core.md");
    if core_file.is_file() {
        let content = std::fs::read_to_string(&core_file)?;
        print_body(&content);
    }

    // Version info
    let claude_dir = home_dir().join(".claude");
    let version = git_version(&claude_dir);
    println!("\n---\n_Ways version: {version}_");

    // Update status from cache
    print_update_status();

    // Dirty file enumeration
    print_dirty_status(&claude_dir);

    // Stamp core marker
    session::stamp_core(session_id);

    Ok(())
}

// ── Helpers ─────────────────────────────────────────────────────

fn extract_field(content: &str, name: &str) -> Option<String> {
    let prefix = format!("{name}:");
    let mut in_fm = false;
    for (i, line) in content.lines().enumerate() {
        if i == 0 && line == "---" {
            in_fm = true;
            continue;
        }
        if in_fm {
            if line == "---" {
                return None;
            }
            if let Some(val) = line.strip_prefix(&prefix) {
                let val = val.trim();
                if !val.is_empty() {
                    return Some(val.to_string());
                }
            }
        }
    }
    None
}

fn print_body(content: &str) {
    let mut fm_count = 0;
    for line in content.lines() {
        if line == "---" {
            fm_count += 1;
            continue;
        }
        if fm_count >= 2 {
            println!("{line}");
        }
    }
}

fn print_check_sections(content: &str, include_anchor: bool) {
    let mut fm_count = 0;
    let mut section = String::new();

    for line in content.lines() {
        if line == "---" {
            fm_count += 1;
            continue;
        }
        if fm_count < 2 {
            continue;
        }

        if line.starts_with("## anchor") {
            section = "anchor".to_string();
            continue;
        }
        if line.starts_with("## check") {
            section = "check".to_string();
            continue;
        }
        if line.starts_with("## ") {
            section = "other".to_string();
            continue;
        }

        if section == "check" || (section == "anchor" && include_anchor) {
            println!("{line}");
        }
    }
}

fn run_macro(macro_file: &Path) -> Option<String> {
    let output = Command::new("bash")
        .arg(macro_file)
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    } else {
        None
    }
}

fn is_project_trusted(project_dir: &str) -> bool {
    let trust_file = home_dir().join(".claude/trusted-project-macros");
    if let Ok(content) = std::fs::read_to_string(&trust_file) {
        content.lines().any(|line| line.trim() == project_dir)
    } else {
        false
    }
}

fn compute_tree_metrics(
    way_id: &str,
    session_id: &str,
) -> (u32, Option<String>, Option<u64>, Option<u64>) {
    let mut depth = 0u32;
    let mut parent_id: Option<String> = None;
    let mut parent_epoch: Option<u64> = None;
    let mut epoch_from_parent: Option<u64> = None;
    let current_epoch = session::get_epoch(session_id);

    let mut path = way_id.to_string();
    while let Some(idx) = path.rfind('/') {
        path = path[..idx].to_string();
        if session::way_is_shown(&path, session_id) {
            depth += 1;
            if parent_id.is_none() {
                parent_id = Some(path.clone());
                let pe = session::get_way_epoch(&path, session_id);
                parent_epoch = Some(pe);
                epoch_from_parent = Some(current_epoch.saturating_sub(pe));
            }
        }
    }

    (depth, parent_id, parent_epoch, epoch_from_parent)
}

fn count_siblings(way_id: &str, project_dir: &str, session_id: &str) -> (u32, u32) {
    let parent_path = match way_id.rfind('/') {
        Some(idx) => &way_id[..idx],
        None => return (0, 0),
    };

    let mut total = 0u32;
    let mut fired = 0u32;

    let bases = [
        PathBuf::from(project_dir).join(".claude/ways"),
        home_dir().join(".claude/hooks/ways"),
    ];

    for base in &bases {
        let parent_dir = base.join(parent_path);
        if !parent_dir.is_dir() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(&parent_dir) {
            for entry in entries.flatten() {
                if !entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    continue;
                }
                let sib_name = entry.file_name().to_string_lossy().to_string();
                let sib_id = format!("{parent_path}/{sib_name}");
                // Check it has a way file
                if session::resolve_way_file(&sib_id, project_dir).is_some() {
                    total += 1;
                    if session::way_is_shown(&sib_id, session_id) {
                        fired += 1;
                    }
                }
            }
        }
    }

    (total, fired)
}

fn git_version(repo: &Path) -> String {
    let output = Command::new("git")
        .args(["-C", &repo.display().to_string(), "describe", "--tags", "--match", "v*", "--always", "--dirty"])
        .output();

    let raw = match output {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        }
        _ => return "unknown".to_string(),
    };

    let (describe, is_dirty) = if raw.ends_with("-dirty") {
        (raw.trim_end_matches("-dirty"), true)
    } else {
        (raw.as_str(), false)
    };

    // Parse: "v0.1.0-29-ge0841be" or "v0.1.0" or "e0841be"
    let version = if let Some(caps) = parse_git_describe(describe) {
        if caps.distance > 0 {
            format!("{} + {} commits ({})", caps.tag, caps.distance, caps.hash)
        } else {
            format!("{} (release)", caps.tag)
        }
    } else if describe.starts_with('v') {
        format!("{describe} (release)")
    } else {
        describe.to_string()
    };

    if is_dirty {
        format!("{version} · dirty")
    } else {
        version
    }
}

struct GitDescribe {
    tag: String,
    distance: u32,
    hash: String,
}

fn parse_git_describe(s: &str) -> Option<GitDescribe> {
    // "v0.1.0-29-ge0841be"
    let last_dash = s.rfind('-')?;
    let hash = &s[last_dash + 1..];
    if !hash.starts_with('g') {
        return None;
    }
    let rest = &s[..last_dash];
    let second_dash = rest.rfind('-')?;
    let distance: u32 = rest[second_dash + 1..].parse().ok()?;
    let tag = &rest[..second_dash];
    Some(GitDescribe {
        tag: tag.to_string(),
        distance,
        hash: hash[1..].to_string(), // strip 'g' prefix
    })
}

fn print_update_status() {
    let uid = unsafe { libc_getuid() };
    let cache_file = format!("/tmp/.claude-config-update-state-{uid}");
    let content = match std::fs::read_to_string(&cache_file) {
        Ok(c) => c,
        Err(_) => return,
    };

    let get = |key: &str| -> Option<String> {
        content
            .lines()
            .find(|l| l.starts_with(&format!("{key}=")))
            .map(|l| l[key.len() + 1..].to_string())
    };

    let cached_type = get("type").unwrap_or_default();
    let behind: u32 = get("behind").and_then(|s| s.parse().ok()).unwrap_or(0);
    let has_upstream = get("has_upstream").unwrap_or_default() == "true";
    let upstream_repo = "aaronsb/claude-code-config";

    if behind == 0 {
        return;
    }

    println!();
    match cached_type.as_str() {
        "clone" => {
            println!("**{behind} commit(s) behind origin/main.** Run: `cd ~/.claude && git pull`");
        }
        "fork" | "renamed_clone" => {
            if has_upstream {
                println!("**Behind {upstream_repo}.** Run: `cd ~/.claude && git fetch upstream && git merge upstream/main`");
            } else {
                println!("**Behind {upstream_repo}.** First add upstream, then sync:");
                println!("`git -C ~/.claude remote add upstream https://github.com/{upstream_repo}`");
                println!("`cd ~/.claude && git fetch upstream && git merge upstream/main`");
            }
        }
        "plugin" => {
            let installed = get("installed").unwrap_or_default();
            let latest = get("latest").unwrap_or_default();
            println!("**Plugin update available (v{installed} -> v{latest}).** Run: `/plugin update disciplined-methodology`");
        }
        _ => {}
    }
}

fn print_dirty_status(claude_dir: &Path) {
    let output = Command::new("git")
        .args(["-C", &claude_dir.display().to_string(), "status", "--short"])
        .output();

    let files: Vec<String> = match output {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.split_whitespace().last().unwrap_or("").to_string())
                .collect()
        }
        _ => return,
    };

    if files.is_empty() {
        return;
    }

    let count = files.len();
    println!();
    if count >= 4 {
        println!("**Uncommitted local changes ({count} files)** — not tracked by git.");
        println!("Other sessions won't see these. Commit to keep, or discard to match remote.");
    } else {
        let s = if count != 1 { "s" } else { "" };
        println!("**Uncommitted local changes ({count} file{s}):**");
    }

    let max_show = 5;
    for f in files.iter().take(max_show) {
        println!("- `{f}`");
    }
    if count > max_show {
        println!("- ... and {} more", count - max_show);
    }
    if count < 4 {
        println!("\n_Run `git -C ~/.claude status` to review._");
    }
}

/// Get uid without pulling in libc crate.
fn libc_getuid() -> u32 {
    #[cfg(unix)]
    unsafe {
        extern "C" {
            fn getuid() -> u32;
        }
        getuid()
    }
    #[cfg(not(unix))]
    0
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

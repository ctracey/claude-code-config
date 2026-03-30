use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::frontmatter;

pub fn run(path: String, jaccard: bool) -> Result<()> {
    let ways_root = home_dir().join(".claude/hooks/ways");
    let tree_path = resolve_path(&path, &ways_root)?;
    let rel_root = tree_path
        .strip_prefix(&ways_root)
        .unwrap_or(&tree_path)
        .display()
        .to_string();

    let files = find_way_files(&tree_path)?;

    if jaccard {
        println!("JACCARD_ROOT\t{rel_root}");
        print_jaccard(&tree_path, &ways_root, &files)?;
    } else {
        println!("TREE_ROOT\t{rel_root}");
        print_tree(&tree_path, &ways_root, &files)?;
    }

    Ok(())
}

struct WayInfo {
    path: PathBuf,
    is_check: bool,
    depth: usize,
    threshold: Option<f64>,
    vocab_count: usize,
    tokens: usize,
}

fn print_tree(tree_path: &Path, ways_root: &Path, files: &[PathBuf]) -> Result<()> {
    for file in files {
        let info = analyze_file(file, tree_path, ways_root)?;
        let rel = file
            .strip_prefix(ways_root)
            .unwrap_or(file)
            .display();
        let ftype = if info.is_check { "check" } else { "way" };
        let thresh = info.threshold.map_or("none".to_string(), |t| format!("{t}"));

        println!(
            "NODE\t{}\t{}\t{}\t{}\t{}\t{}",
            info.depth, rel, thresh, ftype, info.vocab_count, info.tokens
        );
    }
    Ok(())
}

fn print_jaccard(tree_path: &Path, ways_root: &Path, files: &[PathBuf]) -> Result<()> {
    // Group way files (not checks) by parent directory
    let mut by_parent: HashMap<PathBuf, Vec<(PathBuf, String)>> = HashMap::new();

    for file in files {
        if file
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n.contains(".check."))
        {
            continue;
        }

        let dir = file.parent().unwrap_or(file).to_path_buf();
        let parent = dir.parent().unwrap_or(&dir).to_path_buf();

        // Extract vocabulary
        let content = std::fs::read_to_string(file)?;
        let vocab = extract_vocab_from_content(&content);

        by_parent
            .entry(parent)
            .or_default()
            .push((file.clone(), vocab));
    }

    for (_parent, siblings) in &by_parent {
        if siblings.len() < 2 {
            continue;
        }
        for i in 0..siblings.len() {
            for j in (i + 1)..siblings.len() {
                let score = jaccard_similarity(&siblings[i].1, &siblings[j].1);
                let rel_a = siblings[i]
                    .0
                    .strip_prefix(ways_root)
                    .unwrap_or(&siblings[i].0)
                    .display();
                let rel_b = siblings[j]
                    .0
                    .strip_prefix(ways_root)
                    .unwrap_or(&siblings[j].0)
                    .display();
                println!("PAIR\t{rel_a}\t{rel_b}\t{score:.2}");
            }
        }
    }

    Ok(())
}

fn analyze_file(file: &Path, tree_path: &Path, _ways_root: &Path) -> Result<WayInfo> {
    let is_check = file
        .file_name()
        .and_then(|n| n.to_str())
        .map_or(false, |n| n.contains(".check."));

    let dir = file.parent().unwrap_or(file);
    let subpath = dir.strip_prefix(tree_path).unwrap_or(Path::new(""));
    let depth = if subpath.as_os_str().is_empty() {
        0
    } else {
        subpath.components().count()
    };

    let content = std::fs::read_to_string(file)?;
    let threshold = extract_threshold_from_content(&content);
    let vocab = extract_vocab_from_content(&content);
    let vocab_count = if vocab.is_empty() {
        0
    } else {
        vocab.split_whitespace().count()
    };

    // Token estimate: body bytes / 4
    let body = strip_frontmatter(&content);
    let tokens = body.len() / 4;

    Ok(WayInfo {
        path: file.to_path_buf(),
        is_check,
        depth,
        threshold,
        vocab_count,
        tokens,
    })
}

fn jaccard_similarity(vocab_a: &str, vocab_b: &str) -> f64 {
    let a: HashSet<&str> = vocab_a.split_whitespace().collect();
    let b: HashSet<&str> = vocab_b.split_whitespace().collect();
    let union = a.union(&b).count();
    if union == 0 {
        return 0.0;
    }
    let intersection = a.intersection(&b).count();
    intersection as f64 / union as f64
}

fn find_way_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        // Check frontmatter
        if let Ok(content) = std::fs::read_to_string(path) {
            if content.starts_with("---\n") {
                files.push(path.to_path_buf());
            }
        }
    }
    files.sort();
    Ok(files)
}

fn resolve_path(input: &str, ways_root: &Path) -> Result<PathBuf> {
    let p = Path::new(input);
    if p.is_absolute() && p.is_dir() {
        return Ok(p.to_path_buf());
    }

    // Try relative to ways_root
    let candidate = ways_root.join(input);
    if candidate.is_dir() {
        return Ok(candidate);
    }

    // Search recursively
    let mut matches = Vec::new();
    for entry in WalkDir::new(ways_root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir()
            && entry.file_name().to_str() == Some(input)
        {
            matches.push(entry.path().to_path_buf());
        }
    }

    match matches.len() {
        0 => anyhow::bail!("cannot resolve '{input}'"),
        1 => Ok(matches.into_iter().next().unwrap()),
        _ => {
            eprintln!("ambiguous: multiple matches for '{input}':");
            for m in &matches {
                eprintln!("  {}", m.display());
            }
            anyhow::bail!("ambiguous path '{input}'")
        }
    }
}

fn extract_vocab_from_content(content: &str) -> String {
    extract_field(content, "vocabulary").unwrap_or_default()
}

fn extract_threshold_from_content(content: &str) -> Option<f64> {
    extract_field(content, "threshold")?.parse().ok()
}

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
                return Some(val.trim().to_string());
            }
        }
    }
    None
}

fn strip_frontmatter(content: &str) -> String {
    let mut lines = content.lines();
    if lines.next() != Some("---") {
        return content.to_string();
    }
    let mut past_fm = false;
    let mut body = Vec::new();
    for line in lines {
        if !past_fm && line == "---" {
            past_fm = true;
            continue;
        }
        if past_fm {
            body.push(line);
        }
    }
    body.join("\n")
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

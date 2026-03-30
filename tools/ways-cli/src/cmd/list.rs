//! List ways triggered in the current session.
//! Replaces list-triggered.sh (71 lines).

use anyhow::Result;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub fn run(session: Option<&str>) -> Result<()> {
    let ways_dir = home_dir().join(".claude/hooks/ways");
    let pattern = match session {
        Some(id) => format!("/tmp/.claude-way-*-{id}"),
        None => "/tmp/.claude-way-*".to_string(),
    };

    let markers: Vec<PathBuf> = glob::glob(&pattern)
        .map(|paths| paths.filter_map(|p| p.ok()).collect())
        .unwrap_or_default();

    if markers.is_empty() {
        println!("No ways triggered yet this session.");
        return Ok(());
    }

    println!("## Triggered Ways\n");

    let mut seen = BTreeSet::new();

    for marker in &markers {
        let name = marker
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Strip prefix: .claude-way-{way-name-dashed}-{uuid}
        let name = name.strip_prefix(".claude-way-").unwrap_or(name);

        // Skip non-way markers (tokens, epoch, metrics)
        if name.starts_with("tokens-") || name.starts_with("epoch-") || name.starts_with("metrics-") {
            continue;
        }

        // UUID is last 5 hyphen-separated segments
        let parts: Vec<&str> = name.split('-').collect();
        if parts.len() < 6 {
            continue;
        }

        // Way path is everything before the UUID (last 5 segments)
        let way_parts = &parts[..parts.len() - 5];
        let way_path = way_parts.join("/");

        if !seen.insert(way_path.clone()) {
            continue;
        }

        // Try to find the way file and extract title
        let way_dir = ways_dir.join(&way_path);
        let title = find_way_title(&way_dir);

        match title {
            Some(t) => println!("- **{way_path}**: {t}"),
            None => println!("- **{way_path}** _(project-local)_"),
        }
    }

    Ok(())
}

fn find_way_title(dir: &PathBuf) -> Option<String> {
    if !dir.is_dir() {
        return None;
    }
    for entry in std::fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let name = path.file_name()?.to_str()?;
        if name.contains(".check.") {
            continue;
        }
        let content = std::fs::read_to_string(&path).ok()?;
        for line in content.lines() {
            if let Some(title) = line.strip_prefix("# ") {
                return Some(title.to_string());
            }
        }
    }
    None
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

//! Content rendering utilities — pure functions for file processing.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Extract a YAML frontmatter field value by name.
pub(crate) fn extract_field(content: &str, name: &str) -> Option<String> {
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

/// Print markdown body (everything after YAML frontmatter).
pub(crate) fn print_body(content: &str) {
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

/// Print check file sections (anchor and/or check).
pub(crate) fn print_check_sections(content: &str, include_anchor: bool) {
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

/// Execute a macro shell script and return its stdout.
pub(crate) fn run_macro(macro_file: &Path) -> Option<String> {
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

/// Check whether a project directory is in the trusted-project-macros list.
pub(crate) fn is_project_trusted(project_dir: &str) -> bool {
    let trust_file = home_dir().join(".claude/trusted-project-macros");
    if let Ok(content) = std::fs::read_to_string(&trust_file) {
        content.lines().any(|line| line.trim() == project_dir)
    } else {
        false
    }
}

/// Resolve the user's home directory from $HOME.
pub(crate) fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

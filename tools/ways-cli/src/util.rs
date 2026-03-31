//! Shared utility functions used across multiple modules.

use std::path::PathBuf;

/// Home directory from $HOME, falling back to /tmp.
pub fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

/// Detect the project root by walking up from cwd looking for .claude/settings.json or CLAUDE.md.
pub fn detect_project_dir() -> Option<String> {
    let cwd = std::env::current_dir().ok()?;
    let mut dir = cwd.as_path();
    loop {
        let claude_dir = dir.join(".claude");
        if claude_dir.is_dir()
            && (claude_dir.join("settings.json").exists()
                || dir.join("CLAUDE.md").exists()
                || claude_dir.join("settings.local.json").exists())
        {
            return Some(dir.to_string_lossy().to_string());
        }
        dir = dir.parent()?;
    }
}

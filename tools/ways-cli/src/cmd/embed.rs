use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn run(query: String, corpus: Option<String>, model: Option<String>) -> Result<()> {
    let corpus_path = corpus.unwrap_or_else(|| default_corpus().to_string_lossy().to_string());
    let model_path = model.unwrap_or_else(|| default_model().to_string_lossy().to_string());

    let embed_bin = find_way_embed()
        .context("way-embed binary not found. Run `make setup` to install.")?;

    let output = Command::new(&embed_bin)
        .args(["match", "--corpus", &corpus_path, "--model", &model_path, "--query", &query])
        .output()
        .with_context(|| format!("running {}", embed_bin.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("way-embed match failed: {stderr}");
    }

    // Pass through stdout (id<TAB>score lines)
    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn find_way_embed() -> Option<PathBuf> {
    let xdg = xdg_cache_dir().join("claude-ways/user/way-embed");
    if xdg.is_file() {
        return Some(xdg);
    }
    let bin = home_dir().join(".claude/bin/way-embed");
    if bin.is_file() {
        return Some(bin);
    }
    None
}

fn default_corpus() -> PathBuf {
    xdg_cache_dir().join("claude-ways/user/ways-corpus.jsonl")
}

fn default_model() -> PathBuf {
    xdg_cache_dir().join("claude-ways/user/minilm-l6-v2.gguf")
}

use crate::util::home_dir;

fn xdg_cache_dir() -> PathBuf {
    std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".cache"))
}

//! Scan ways and output matched content — replaces hook scan loops.
//!
//! Combines file walking, frontmatter extraction, matching (pattern + semantic),
//! scope/precondition gating, parent-threshold lowering, and show (display).

mod candidates;
mod scoring;

use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;

use crate::session;

use candidates::{check_when, collect_candidates, collect_checks};
use scoring::{
    batch_bm25_score, batch_embed_score, capture_show_check, capture_show_way, default_project,
};

pub(crate) struct WayCandidate {
    pub id: String,
    pub path: PathBuf,
    pub pattern: Option<String>,
    pub commands: Option<String>,
    pub files: Option<String>,
    pub description: String,
    pub vocabulary: String,
    pub threshold: f64,
    pub scope: String,
    pub when_project: Option<String>,
    pub when_file_exists: Option<String>,
}

// ── Prompt scan ─────────────────────────────────────────────────

pub fn prompt(query: &str, session_id: &str, project: Option<&str>) -> Result<()> {
    let project_dir = project
        .map(|s| s.to_string())
        .unwrap_or_else(default_project);

    // Bump epoch
    session::bump_epoch(session_id);

    let scope = session::detect_scope(session_id);
    let candidates = collect_candidates(&project_dir);

    // Batch semantic scoring
    let bm25_matches = batch_bm25_score(query);
    let embed_matches = batch_embed_score(query);

    for way in &candidates {
        if !session::scope_matches(&way.scope, &scope) {
            continue;
        }
        if !check_when(&way.when_project, &way.when_file_exists, &project_dir) {
            continue;
        }

        // Parent-aware threshold lowering
        let effective_threshold = parent_threshold(&way.id, way.threshold, session_id);

        // Additive matching: pattern OR semantic
        let channel = match_prompt(
            query,
            &way.pattern,
            &way.id,
            effective_threshold,
            &bm25_matches,
            &embed_matches,
        );

        if let Some(trigger) = channel {
            let _ = crate::cmd::show::way(&way.id, session_id, &trigger);
        }
    }

    Ok(())
}

// ── Command scan ────────────────────────────────────────────────

pub fn command(
    cmd: &str,
    description: Option<&str>,
    session_id: &str,
    project: Option<&str>,
) -> Result<()> {
    let project_dir = project
        .map(|s| s.to_string())
        .unwrap_or_else(default_project);

    session::bump_epoch(session_id);
    let scope = session::detect_scope(session_id);
    let candidates = collect_candidates(&project_dir);

    let mut context = String::new();

    // Way matching: commands regex + pattern regex
    for way in &candidates {
        if !session::scope_matches(&way.scope, &scope) {
            continue;
        }
        if !check_when(&way.when_project, &way.when_file_exists, &project_dir) {
            continue;
        }

        let mut matched = false;

        if let Some(ref cmds_pattern) = way.commands {
            if regex_matches(cmds_pattern, cmd) {
                matched = true;
            }
        }

        if !matched {
            if let Some(ref desc) = description {
                if let Some(ref pat) = way.pattern {
                    if regex_matches(pat, &desc.to_lowercase()) {
                        matched = true;
                    }
                }
            }
        }

        if matched {
            let out = capture_show_way(&way.id, session_id, "bash");
            if !out.is_empty() {
                context.push_str(&out);
            }
        }
    }

    // Check matching: commands regex + semantic scoring
    let checks = collect_checks(&project_dir);
    let query_for_checks = format!(
        "{} {}",
        cmd,
        description.unwrap_or("")
    );

    // Batch BM25 for check scoring
    let bm25_matches = batch_bm25_score(&query_for_checks);

    for check in &checks {
        if !session::scope_matches(&check.scope, &scope) {
            continue;
        }
        if !check_when(&check.when_project, &check.when_file_exists, &project_dir) {
            continue;
        }

        let mut match_score: f64 = 0.0;

        if let Some(ref cmds_pattern) = check.commands {
            if regex_matches(cmds_pattern, cmd) {
                match_score = 3.0;
            }
        }

        if match_score == 0.0 && !check.description.is_empty() && !check.vocabulary.is_empty() {
            if let Some(score) = bm25_matches.iter().find(|(id, _)| *id == check.id).map(|(_, s)| *s) {
                if score > 0.0 {
                    match_score = score;
                }
            }
        }

        if match_score > 0.0 {
            let out = capture_show_check(&check.id, session_id, "bash", match_score);
            if !out.is_empty() {
                context.push_str(&out);
            }
        }
    }

    // Output JSON for PreToolUse
    if !context.is_empty() {
        println!(
            "{}",
            serde_json::json!({
                "decision": "approve",
                "additionalContext": context
            })
        );
    }

    Ok(())
}

// ── File scan ───────────────────────────────────────────────────

pub fn file(filepath: &str, session_id: &str, project: Option<&str>) -> Result<()> {
    let project_dir = project
        .map(|s| s.to_string())
        .unwrap_or_else(default_project);

    session::bump_epoch(session_id);
    let scope = session::detect_scope(session_id);
    let candidates = collect_candidates(&project_dir);

    let mut context = String::new();

    for way in &candidates {
        if !session::scope_matches(&way.scope, &scope) {
            continue;
        }
        if !check_when(&way.when_project, &way.when_file_exists, &project_dir) {
            continue;
        }

        if let Some(ref files_pattern) = way.files {
            if regex_matches(files_pattern, filepath) {
                let out = capture_show_way(&way.id, session_id, "file");
                if !out.is_empty() {
                    context.push_str(&out);
                }
            }
        }
    }

    // Check matching for files
    let checks = collect_checks(&project_dir);
    let bm25_matches = batch_bm25_score(filepath);

    for check in &checks {
        if !session::scope_matches(&check.scope, &scope) {
            continue;
        }
        if !check_when(&check.when_project, &check.when_file_exists, &project_dir) {
            continue;
        }

        let mut match_score: f64 = 0.0;

        if let Some(ref files_pattern) = check.files {
            if regex_matches(files_pattern, filepath) {
                match_score = 3.0;
            }
        }

        if match_score == 0.0 && !check.description.is_empty() && !check.vocabulary.is_empty() {
            if let Some(score) = bm25_matches.iter().find(|(id, _)| *id == check.id).map(|(_, s)| *s) {
                if score > 0.0 {
                    match_score = score;
                }
            }
        }

        if match_score > 0.0 {
            let out = capture_show_check(&check.id, session_id, "file", match_score);
            if !out.is_empty() {
                context.push_str(&out);
            }
        }
    }

    if !context.is_empty() {
        println!(
            "{}",
            serde_json::json!({
                "decision": "approve",
                "additionalContext": context
            })
        );
    }

    Ok(())
}

// ── Matching ────────────────────────────────────────────────────

fn match_prompt(
    query: &str,
    pattern: &Option<String>,
    way_id: &str,
    threshold: f64,
    bm25: &[(String, f64)],
    embed: &[(String, f64)],
) -> Option<String> {
    // Channel 1: Regex pattern
    if let Some(ref pat) = pattern {
        if regex_matches(pat, query) {
            return Some("keyword".to_string());
        }
    }

    // Channel 2: Embedding (highest priority semantic)
    if embed.iter().any(|(id, _)| id == way_id) {
        return Some("semantic:embedding".to_string());
    }

    // Channel 3: BM25
    if let Some((_, score)) = bm25.iter().find(|(id, _)| id == way_id) {
        if *score >= threshold {
            return Some("semantic:bm25".to_string());
        }
    }

    None
}

fn parent_threshold(way_id: &str, threshold: f64, session_id: &str) -> f64 {
    let mut path = way_id.to_string();
    while let Some(idx) = path.rfind('/') {
        path = path[..idx].to_string();
        if session::way_is_shown(&path, session_id) {
            return threshold * 0.8;
        }
    }
    threshold
}

fn regex_matches(pattern: &str, text: &str) -> bool {
    Regex::new(pattern)
        .map(|re| re.is_match(text))
        .unwrap_or(false)
}

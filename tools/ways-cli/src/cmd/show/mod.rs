//! Display ways, checks, and core guidance — session-aware, idempotent.
//!
//! Replaces: show-way.sh, show-check.sh, show-core.sh

mod helpers;
mod metrics;

use anyhow::Result;
use serde_json::json;
use std::path::Path;

use crate::session;
use helpers::{extract_field, home_dir, is_project_trusted, print_body, print_check_sections, run_macro};
use metrics::{compute_tree_metrics, count_siblings, git_version, print_dirty_status, print_update_status};

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

use anyhow::{bail, Context, Result};
use std::path::PathBuf;

use crate::table::{Table, Align};

struct CorpusEntry {
    id: String,
    embedding: Vec<f32>,
}

pub fn run(id: String, threshold: f64, corpus: Option<String>, _model: Option<String>) -> Result<()> {
    let corpus_path = corpus.unwrap_or_else(|| default_corpus().to_string_lossy().to_string());

    let entries = load_embeddings(&corpus_path)
        .with_context(|| format!("loading embeddings from {corpus_path}"))?;

    if entries.is_empty() {
        bail!("no entries with embeddings found in corpus");
    }

    if id == "all" {
        // Full NxN matrix
        print_matrix(&entries, threshold);
    } else {
        // Single way vs all others
        // Try exact match, then suffix match (e.g., "code/quality" matches "softwaredev/code/quality")
        let target = entries.iter().find(|e| e.id == id)
            .or_else(|| entries.iter().find(|e| e.id.ends_with(&format!("/{id}"))));
        let target = match target {
            Some(t) => t,
            None => bail!("way '{id}' not found in corpus"),
        };

        let target_id = &target.id;
        let mut scores: Vec<(&str, f64)> = entries
            .iter()
            .filter(|e| e.id != *target_id)
            .map(|e| (e.id.as_str(), cosine_similarity(&target.embedding, &e.embedding) as f64))
            .filter(|(_, s)| *s >= threshold)
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if scores.is_empty() {
            eprintln!("no siblings above threshold {threshold}");
        } else {
            println!();
            let mut t = Table::new(&["Way A", "Way B", "Cosine"]);
            t.max_width(0, 36);
            t.max_width(1, 36);
            t.align(2, Align::Right);
            for (other_id, score) in &scores {
                t.add_owned(vec![
                    target_id.clone(),
                    other_id.to_string(),
                    format!("{score:.4}"),
                ]);
            }
            t.print();
            println!();
        }
    }

    Ok(())
}

fn print_matrix(entries: &[CorpusEntry], threshold: f64) {
    let mut t = Table::new(&["Way A", "Way B", "Cosine"]);
    t.max_width(0, 36);
    t.max_width(1, 36);
    t.align(2, Align::Right);

    for (i, a) in entries.iter().enumerate() {
        for b in entries.iter().skip(i + 1) {
            let score = cosine_similarity(&a.embedding, &b.embedding) as f64;
            if score >= threshold {
                t.add_owned(vec![
                    a.id.clone(),
                    b.id.clone(),
                    format!("{score:.4}"),
                ]);
            }
        }
    }

    println!();
    t.print();
    println!();
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    // Vectors are already L2-normalized, so dot product = cosine similarity
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn load_embeddings(path: &str) -> Result<Vec<CorpusEntry>> {
    let content = std::fs::read_to_string(path)?;
    let mut entries = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let val: serde_json::Value = serde_json::from_str(line)?;
        let id = val["id"].as_str().unwrap_or("").to_string();

        // Only include entries that have pre-computed embeddings
        if let Some(arr) = val["embedding"].as_array() {
            let embedding: Vec<f32> = arr
                .iter()
                .filter_map(|v| v.as_f64().map(|f| f as f32))
                .collect();
            if !embedding.is_empty() {
                entries.push(CorpusEntry { id, embedding });
            }
        }
    }

    Ok(entries)
}

fn default_corpus() -> PathBuf {
    let xdg = std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".cache"))
                .unwrap_or_else(|_| PathBuf::from("/tmp"))
        });
    xdg.join("claude-ways/user/ways-corpus.jsonl")
}

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::bm25;

pub fn run(query: String, corpus: Option<String>) -> Result<()> {
    let corpus_path = corpus
        .unwrap_or_else(|| default_corpus_path().to_string_lossy().to_string());

    let stemmer = bm25::new_stemmer();
    let corpus = bm25::load_corpus_jsonl(&corpus_path, &stemmer)
        .with_context(|| format!("loading corpus {corpus_path}"))?;

    if corpus.docs.is_empty() {
        eprintln!("error: empty corpus");
        std::process::exit(1);
    }

    let query_tokens = bm25::tokenize(&query, &stemmer);

    // Score all documents
    let mut scored: Vec<(usize, f64)> = corpus
        .docs
        .iter()
        .enumerate()
        .map(|(i, doc)| (i, corpus.bm25_score(doc, &query_tokens)))
        .collect();

    // Sort descending by score
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Output matches above per-document threshold
    let mut printed = 0;
    for (idx, score) in &scored {
        let doc = &corpus.docs[*idx];
        let threshold = if doc.threshold > 0.0 { doc.threshold } else { 2.0 };

        if *score >= threshold {
            // Truncate description for display
            let snippet: String = if doc.description.len() > 56 {
                format!("{}...", &doc.description[..56])
            } else {
                doc.description.clone()
            };
            println!("{}\t{:.4}\t{}", doc.id, score, snippet);
            printed += 1;
        }
    }

    if printed == 0 {
        eprintln!("no matches above threshold");
        std::process::exit(1);
    }

    Ok(())
}

fn default_corpus_path() -> PathBuf {
    let xdg = std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".cache"))
                .unwrap_or_else(|_| PathBuf::from("/tmp"))
        });
    xdg.join("claude-ways/user/ways-corpus.jsonl")
}

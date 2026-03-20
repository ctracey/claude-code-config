---
status: Draft
date: 2026-03-20
deciders:
  - aaronsb
  - claude
related:
  - ADR-014
---

# ADR-107: Way-Match Corpus, Batch Mode, and Locale Support

## Context

ADR-014 introduced the `way-match` BM25 binary as a replacement for gzip NCD semantic matching. The binary works well — 0 false positives across 74 test fixtures, good recall on natural language prompts — but three limitations are now visible after scaling from 7 semantic ways to 55+.

### Stale IDF corpus

`way-match pair` mode (what the runtime scanners use) computes IDF against a hardcoded `BUILTIN_WAYS[]` array of 7 way descriptions baked into the C source (line 352-368 of `way-match.c`). This was the entire semantic corpus when the binary was written. With 55+ semantic ways, the IDF calculations are wrong — term rarity is computed against 7 documents when the real corpus is 55+. A term like "test" that appears in many ways still gets high IDF because the builtin only has one way containing it.

The binary already has a `score` mode that loads an external JSONL corpus for correct IDF. But `pair` mode — the one called 55 times per prompt by the scanners — ignores it.

### N process spawns per prompt

Each scanner (`check-prompt.sh`, `check-bash-pre.sh`, `check-file-pre.sh`) calls `way-match pair` once per semantic way. At 55 semantic ways, that's 55 process spawns per user prompt. Each spawn re-initializes the Snowball stemmer, re-tokenizes the query, and re-builds the (wrong) IDF from the builtin corpus. The `score` command already supports batch scoring — one invocation, all ways ranked — but the scanners don't use it.

### English-only

The Snowball Porter2 stemmer (`stem_UTF_8_english.h`) and the stopword list are English. The ways system is currently English, but Claude Code has a `language` setting and users in non-English locales would benefit from ways written in their language with appropriate stemming.

### Security boundary

Any evolution of the binary must respect a hard constraint: **runtime scanners must never write to `~/.claude/`**. Way matching happens during normal project work in arbitrary repositories. Writing to the user's config directory during a prompt evaluation would be a form of supply-chain poisoning — a project-level operation mutating user-global state. All writes to `~/.claude/` (corpus generation, binary rebuilds) are authoring operations invoked explicitly by the user.

## Decision

Evolve `way-match` in three phases, each independently shippable:

### Phase 1: External corpus file

Generate `hooks/ways/ways-corpus.jsonl` from all semantic way.md files. This file is:

- **Git-tracked** — committed to the repo like `bin/way-match`. It's a build artifact, not a runtime cache.
- **Regenerated during authoring** — `lint-ways.sh` and `/ways-tests` scoring regenerate it before any operation that reads it. This ensures the linter and scorer always see the current state.
- **Read-only at runtime** — scanners read the file if present, fall back to `BUILTIN_WAYS[]` if absent. No writes during prompt evaluation.

The corpus generator is a shell script (or a `way-match generate` subcommand) that:
1. Finds all `way.md` files with `description:` + `vocabulary:` fields
2. Extracts id, description, vocabulary, threshold from frontmatter
3. Emits one JSON line per way to `ways-corpus.jsonl`

`way-match pair` mode gains an optional `--corpus` flag. When provided, it loads the external corpus for IDF instead of `BUILTIN_WAYS[]`. The scanners pass `--corpus` if the file exists.

### Phase 2: Batch scoring

Replace N `pair` calls with a single `score` call per scanner. The scanner:
1. Calls `way-match score --corpus ways-corpus.jsonl --query "the prompt"`
2. Gets back all matching ways above their respective thresholds (tab-delimited)
3. Fires `show-way.sh` for each match

This reduces 55 process spawns to 1. The `score` command already exists and handles per-document thresholds. The scanner refactoring is in shell, not C.

The `pair` command remains available for single-way testing (used by `/ways-tests score`).

### Phase 3: Locale-aware ways

Introduce locale variants alongside `way.md`:

```
hooks/ways/softwaredev/code/security/
  way.md        # default (English)
  way-es.md     # Spanish
  way-fr.md     # French
```

**Schema addition:** `locale:` field in `frontmatter-schema.yaml`. The linter validates that locale files have valid locale codes and matching structure.

**Scanner behavior:** Check Claude Code's `language` setting (available via environment or settings.json). Select `way-{locale}.md` if it exists, fall back to `way.md`. A Spanish-speaking user gets `way-es.md` with Spanish description/vocabulary; the stemmer uses Snowball's Spanish module.

**Binary changes:** Snowball ships stemmers for 20+ languages. The binary gains a `--language` flag that selects the stemmer. Stopwords become a per-language array (Snowball provides these too). The corpus JSONL gains a `locale` field so IDF is computed within-locale.

**Scope:** We only build locale variants for ways that have volunteer translators. Most ways stay English-only. The system degrades gracefully — no locale file means English, which is the current behavior.

## Consequences

### Positive

- Phase 1: Correct IDF across the real corpus. Immediate accuracy improvement with minimal code change.
- Phase 2: 55x fewer process spawns per prompt. Measurable latency reduction on every user interaction.
- Phase 3: Ways system becomes accessible to non-English users. Same architecture, same matching quality, different language.
- Each phase ships independently — no big-bang migration.
- The trust chain from ADR-014 is preserved: tier 0 (bash fallback) → tier 1 (source-auditable binary) → tier 1.5 (source-auditable binary + tracked corpus).

### Negative

- Phase 1: Another file to track in git. Regeneration adds a step to the authoring workflow (mitigated by integration with lint/score).
- Phase 2: Scanner refactoring changes the output parsing contract between shell and binary. Needs careful testing.
- Phase 3: Significant binary complexity (multi-stemmer, per-language stopwords). Translation maintenance burden grows with each locale.
- The corpus file can go stale if someone modifies ways without running the linter. This is a process discipline issue, not an architectural one — same as forgetting to rebuild after changing source.

### Neutral

- `BUILTIN_WAYS[]` stays in the binary as a fallback. It's wrong but it's better than nothing when the corpus file is absent.
- The `pair` command isn't removed — it's still useful for single-way testing via `/ways-tests`.
- Locale support doesn't require all ways to be translated. English is and remains the default.
- The defensive boundary (no runtime writes to `~/.claude/`) applies to all phases.

## Alternatives Considered

- **Embeddings (fastembed, ONNX, llamafile)**: Would solve the semantic gap ("make it faster" → performance) that BM25 can't handle. Rejected for now — BM25 at 0 FP with 55+ semantic ways is sufficient. Embeddings require shipping a model file (25-130MB), which changes the trust profile from "source-auditable" to "model-auditable." The defensible trigger for reconsidering: measurable false negatives in production that vocabulary tuning cannot fix, across a corpus that has outgrown manual curation. We're not there.
- **Runtime corpus regeneration**: Regen the JSONL on every prompt if any way.md is newer. Rejected — this writes to `~/.claude/` during project work, which is a poisoning vector. The corpus is a build artifact, not a cache.
- **Embed corpus in the binary**: Compile `ways-corpus.jsonl` into the binary as a C array (like `BUILTIN_WAYS[]` but generated). Would eliminate the external file. Rejected — requires recompiling the binary every time a way changes. The current model (binary rarely changes, corpus changes with ways) is more practical.
- **Move to Python/Node for matching**: Would make stemmer/locale swapping trivial (NLTK, etc). Rejected — adds a runtime dependency. The ways system runs on bash + coreutils + one static binary. That property is worth preserving.
- **Use Claude itself for matching**: The model reading the ways has world-class semantic understanding. Could we score relevance in-band? The old `model-match.sh` tried this via `claude -p` subprocesses. Rejected — Anthropic's ToS restricts programmatic Claude subscription usage, and subprocess latency is orders of magnitude worse than BM25. If Claude Code ever exposes an in-process scoring API, this becomes viable.

## References

- ADR-014: TF-IDF/BM25 Binary for Semantic Way Matching (predecessor)
- `tools/way-match/way-match.c`: Current implementation (907 lines)
- `hooks/ways/frontmatter-schema.yaml`: Authoritative field definitions
- [Snowball stemmer project](https://snowballstem.org/): Multi-language stemming algorithms
- Claude Code `language` setting: User locale preference

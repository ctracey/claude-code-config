# Multi-Language Support

Ways supports multilingual matching and output across 52 languages. The system uses two embedding models: a precise English model and a broad multilingual model, routed per-way via frontmatter.

## Setting output language

The agent writes commit messages, comments, and documentation in the configured language. Resolution order:

1. **`ways.json`** `output_language` field — explicit override
2. **Claude Code** `settings.json` `language` field — agent config (project then user)
3. **System locale** (`$LC_ALL` → `$LC_MESSAGES` → `$LANG`)
4. **Default**: `en`

```json
// ways.json — explicit override
{"disabled": [], "output_language": "ja"}
```

```json
// Claude Code settings.json — agent-level
{"language": "japanese"}
```

Setting `output_language: "auto"` skips the override and cascades to Claude Code settings → system locale.

The output language directive is injected via `core.md` at session start. Way content (the guidance text) stays English — the agent reads it fine in any language. Only the file output changes.

## How matching works across languages

Two embedding models handle different matching scenarios:

| Model | File | Size | Languages | Use case |
|-------|------|------|-----------|----------|
| all-MiniLM-L6-v2 | `minilm-l6-v2.gguf` | 21MB | English | Precise EN matching (default) |
| paraphrase-multilingual-MiniLM-L12-v2 | `multilingual-minilm-l12-v2-q8.gguf` | 127MB | 52 | Cross-language and same-language matching |

Both are downloaded by `make setup` and stored in `~/.cache/claude-ways/user/`.

Each way declares which model it uses via the `embed_model` frontmatter field:

```yaml
---
description: security vulnerability scanning
vocabulary: security vulnerability CVE audit
embed_model: en          # default — uses English model
embed_threshold: 0.35
---
```

Ways with `embed_model: multilingual` are scored by the multilingual model against a separate corpus.

## Creating language stubs

A language stub is a frontmatter-only `.{lang}.md` file that provides native-language matching vocabulary for an existing way. The way body stays English — only the matching changes.

```
hooks/ways/softwaredev/code/security/
  security.md           # English way — full body + frontmatter
  security.ja.md        # Japanese stub — frontmatter only, no body
  security.ko.md        # Korean stub — frontmatter only, no body
```

Example stub (`security.ja.md`):

```yaml
---
description: セキュリティ脆弱性スキャンと監査
vocabulary: セキュリティ 脆弱性 CVE 監査 認証 暗号化
embed_model: multilingual
embed_threshold: 0.25
---
```

When a Japanese user types a prompt, the scanner:
1. Matches `security.ja.md`'s frontmatter using the multilingual model
2. Injects `security.md`'s English body (the guidance text)

The agent reads the English guidance and responds in the configured output language.

### Why same-language stubs matter

Cross-language matching (Japanese prompt → English description) scores ~0.69. Same-language matching (Japanese prompt → Japanese description) scores ~0.93. The stub's native-language description dramatically improves matching precision.

| Scenario | Cosine similarity |
|----------|----------------:|
| EN prompt → EN description (baseline) | 0.76 |
| JA prompt → EN description (cross-language) | 0.69 |
| JA prompt → JA description (same-language stub) | 0.93 |

See `docs/architecture/system/multilingual-model-evaluation.md` for full test results.

## Supported languages

Languages are defined in `tools/ways-cli/languages.json`. Each entry specifies:

- **`name`** / **`native`** — display names for normalization
- **`bm25_stemmer`** — Snowball stemmer algorithm name, or `"impossible"` if BM25 cannot support this language

### BM25 feasibility

BM25 is the fallback matching engine when the embedding model is unavailable. It works for languages with whitespace word boundaries and suffix-stripping morphology:

**BM25 works**: Danish, Dutch, English, Finnish, French, German, Greek, Hungarian, Italian, Norwegian, Portuguese, Romanian, Russian, Spanish, Swedish, Turkish

**BM25 impossible**: Arabic, Burmese, Chinese, Georgian, Gujarati, Hebrew, Hindi, Japanese, Korean, Marathi, Mongolian, Thai, Urdu, Vietnamese — and others marked `"impossible"` in `languages.json`

For "impossible" languages, the embedding engine is required — not optional. Without it, only keyword/regex patterns fire.

## Checking language status

```bash
# Language coverage report
ways language

# Filter to a specific language
ways language --filter ja

# Machine-readable
ways language --json

# Engine status with corpus breakdown
ways status
```

`ways status` warns if multilingual ways exist in the corpus but the multilingual model is missing.

## Architecture decisions

- **ADR-107**: Full design rationale — language cascade, dual model approach, matching tiers
- **Evaluation report**: `docs/architecture/system/multilingual-model-evaluation.md` — test data across 11 languages × 3 domains

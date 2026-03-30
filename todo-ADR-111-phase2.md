# ADR-111 Phase 2: Hook Infrastructure Consolidation

**Branch:** `staging/ADR-111`
**Status:** Phase 2 complete. All absorbable scripts consolidated.

## What's done (25 commits on this branch)

### Phase 1 — ways CLI binary (Rust, 3.1 MB, 15 subcommands)
- `lint`, `corpus`, `graph`, `match`, `embed`, `siblings`, `tree`, `provenance`
- `suggest`, `show` (way/check/core), `scan` (prompt/command/file/state)
- `status`, `stats`, `list`, `init`
- Pure Rust BM25 (rust-stemmers), scores match C version within 0.005
- Embedding via subprocess to existing way-embed binary

### Phase 1 tool replacement (-7,869 lines deleted)
- `way-match.c` + `snowball/` (2,110 lines C) → `ways match`
- `generate-corpus.sh` (250 lines) → `ways corpus`
- `lint-ways.sh` (525 lines) → `ways lint`
- `way-tree-analyze.sh` (270 lines) → `ways tree`
- `provenance-scan.py` (330 lines) → `ways provenance`
- `embed-lib.sh` (190 lines) → absorbed into `ways corpus`
- `embed-suggest.sh` (100 lines) → `ways suggest`
- `bin/way-match` (C binary) → `ways match`

### Phase 2 — session state + display
- `session.rs`: markers, epochs, token positions, scope, event logging
- `show-way.sh` (222 lines) → `ways show way`
- `show-check.sh` (169 lines) → `ways show check`
- `show-core.sh` (162 lines) → `ways show core`
- `token-position.sh` (111 lines) → `session.rs`

### Phase 2 — hook rewrite + script absorption
- `check-prompt.sh`: 101 → 27 lines (one `ways scan prompt` call)
- `check-bash-pre.sh`: 142 → 18 lines (one `ways scan command` call)
- `check-file-pre.sh`: 138 → 18 lines (one `ways scan file` call)
- `check-state.sh`: 217 → 17 lines (one `ways scan state` call)
- `match-way.sh` (183 lines) DELETED
- `detect-scope.sh` (39 lines) DELETED
- `epoch.sh` (35 lines) DELETED
- `log-event.sh` (18 lines) DELETED — inlined where still needed
- `embed-status.sh` (301 lines) DELETED → `ways status`
- `stats.sh` (348 lines) DELETED → `ways stats`
- `list-triggered.sh` (71 lines) DELETED → `ways list`
- `check-embedding-staleness.sh` (60 lines) DELETED → `ways corpus --if-stale`
- `init-project-ways.sh` (81 lines) DELETED → `ways init`
- `model-match.sh` (27 lines) DELETED
- `check-smart-trigger.sh` (125 lines) DELETED (broken, needs redesign)

### Infrastructure
- Makefile: build-only (`make ways`, `make setup`, `make install`, `make test`)
- `make install` symlinks to `~/.local/bin/ways` (globally available)
- `way-embed/Makefile` updated to call `ways corpus`
- `settings.json` updated: staleness check → `ways corpus --if-stale`, init → `ways init`
- Code quality: `scan.rs` and `show.rs` split into module directories (all under 500 lines)

### Net: +6,138 / -7,869 lines. 15 subcommands. 595 lines of bash remain.

## What remains (permanent bash — stays by design)

| Script | Lines | Reason |
|--------|-------|--------|
| `macro.sh` | 161 | Runs arbitrary bash (macros are shell by design) |
| `inject-subagent.sh` | 146 | Two-phase subagent injection, complex state |
| `check-task-pre.sh` | 128 | Subagent stash pattern |
| `check-response.sh` | 48 | Response topic extraction |
| `check-prompt.sh` | 27 | Thin dispatcher → `ways scan prompt` |
| `clear-markers.sh` | 26 | Tiny, rm markers |
| `check-file-pre.sh` | 18 | Thin dispatcher → `ways scan file` |
| `check-bash-pre.sh` | 18 | Thin dispatcher → `ways scan command` |
| `check-state.sh` | 17 | Thin dispatcher → `ways scan state` |
| `mark-tasks-active.sh` | 6 | 6 lines, trivial |

## What's next (future sessions)

### Session simulator integration test
- Spec at `tools/ways-cli/tests/SIMULATION-SPEC.md`
- Deterministic replay of synthetic Claude Code sessions
- 8 scenarios covering matching, idempotency, checks, progressive disclosure, scope, re-disclosure
- Build as cargo integration tests

### Cross-compilation + CI
- `cargo-zigbuild` for 4-platform matrix (linux-x86_64, linux-aarch64, darwin-x86_64, darwin-arm64)
- GitHub Actions workflow replacing the way-embed-only CI
- `make release` target producing tarballs with checksums

### Governance consolidation
- `governance.sh` (543 lines) + `provenance-verify.sh` → `ways governance`
- Already calls `ways provenance` for scanning; the report/query modes are the remaining work

### Smart trigger redesign
- Old `check-smart-trigger.sh` deleted (was broken)
- Needs rethinking for the binary architecture — model-confirmed matching could be a `ways scan` flag

### Possible future absorptions
- `inject-subagent.sh` (146) — complex but could become `ways inject`
- `check-task-pre.sh` (128) — could become `ways scan task`
- These are lower priority; the current thin-dispatch pattern works fine

### Ship PRs
- `staging/ADR-110` → `main` (file rename + docs, 10 commits)
- `staging/ADR-111` → `main` (ways CLI + consolidation, 25 commits)
- Or squash-merge both into one PR if preferred

## How to resume

```
git checkout staging/ADR-111
cat .claude/todo-ADR-111-phase2.md
ways --help
make test
```

The ways binary is LIVE — hooks fire against it every message.
All 15 embedding tests pass. Smoke tests pass. BM25 parity confirmed.

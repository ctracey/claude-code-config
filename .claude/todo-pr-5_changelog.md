# PR-5 Changelog — Claude Code subagent workflow

Append-only. One section per task. Records what was decided, changed, or learned during implementation.

---

## Task 1.1 — Define reference doc conventions `2026-03-27`

- Established four doctype suffixes: *(none)*, `_plan`, `_architecture`, `_notes`
- Tracking files scoped to PR/ADR/issue number (e.g. `todo-pr-5.md`)
- Hyphen couples name parts; underscore separates doctype qualifier

## Task 1.2 — Build the task-execution skill `2026-03-27`

- Originally named `task-execution`; renamed to `todo-execute` when noun-first naming convention was adopted (see Task 2.1 notes)
- Skill spawns a fresh implementation subagent per task — main session never implements

## Task 2.1 — Build the todo-list skill `2026-03-27`

- Originally named `show-tasks`; went through `show-todo-list` before settling on `todo-list`
- Noun-first `todo-` prefix adopted here as the convention for all todo skills — distinguishes from Claude Code's native `Task*` tools
- Display format: `✔` + combining strikethrough for done, `▣` for in progress, `□` for not started
- Subtasks indented two spaces; output as plain text (no markdown code block)

## Task 2.2 — Build the todo-plan skill `2026-03-27`

- Decided that plan summary should be concise by default, with explicit invitation to ask for detail or request the full doc
- Pattern applies to all summary skills (`todo-plan`, `todo-notes`)

## Task 2.3 — Build the todo-notes skill `2026-03-27`

- Same summary-first pattern as `todo-plan`

## Task 2.4 — Build the todo-changelog skill `2026-03-27`

- Created `todo-changelog` skill: shows last 3 task entries by default, supports specific task lookup
- Added to notes skill table alongside `todo-notes`
- Follows same summary-first pattern with invitation to ask for more
- Entries include date timestamp in heading

## Task 4.1 — Split _notes and _changelog into separate docs `2026-03-27`

- `_notes` is a stable reference — conventions and agreements that apply across tasks
- `_changelog` is append-only — per-task record of what happened and why
- Motivation: two different readers and lifecycles; merging them made `_notes` harder to scan
- `todo-notes` skill summarises `_notes`; `todo-changelog` surfaces recent entries

## Task 2.5 — Build the todo-report skill `2026-03-27`

- Initial implementation read all three docs and produced a single combined output
- Refactored to delegate to `todo-plan`, `todo-list`, `todo-notes` in sequence — removes duplication, each skill owns its own format
- Added `NEXT STEP` section: identifies the first `[ ]` task top-to-bottom, outputs task number and one-line description
- `todo-list` gained a `TODO LIST` title header at the top of its output

## Task 2.6/2.7/2.8 — Task list additions and numbering fix `2026-03-27`

- Duplicate `2.6` numbering corrected: `todo-add` moved to `2.7`, gitignore consideration added as `2.8`
- Task 7 added: todo file lifecycle and cleanup (tracking strategy, ship flow cleanup step)
- Task 8 added: implementation agent reflection (what to reflect on, capture in summary artifact)
- Task 2.8 decision taken immediately: todo files are documentation, track in git via `.gitignore` allowlist

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

## Task 2.6 — Build the todo-update skill `2026-03-29`

- Skill updates task/subtask checkbox to `done`, `in-progress`, or `reset`
- Rollup logic: re-evaluates parent status after each subtask change — all done → `[x]`, any started → `[-]`, all reset → `[ ]`
- Built without CLI backing — direct markdown edit proved reliable enough; CLI deferred to todo-add (2.7) decision
- Tested: mark done, reset, and re-mark done all behaved correctly with correct parent rollup

## Notes session — Agent research and workflow clarification `2026-03-29`

- Explored existing agents: `task-planner`, `code-reviewer`, `workflow-orchestrator`, `Plan` (built-in), `system-architect`
- ADR workflow distinguished from this workflow: ADR is architectural lead / decision capture; this workflow is task/outcome driven
- `code-reviewer` confirmed as the review subagent for task 6.2 — needs context wiring only
- `task-planner` identified as candidate for task breakdown step in `todo-new` (3.1); key mismatch: it uses ephemeral TodoWrite, this workflow uses persistent markdown — translation is main session's responsibility
- `Plan` (built-in) identified as candidate for architecture analysis step in `todo-new`, complementing `task-planner`
- Added notes: CLI tool for todo management, packaging options, ways/fresh agent sessions, workflow components inventory
- Task 1.4 marked done (ADR method assessed, fit documented in notes)

## Task 3.1 — Build the todo-begin skill `2026-03-29`

- Skill named `todo-begin` (over `todo-new`, `todo-init`) — "begin" signals intent clearly without implying item-level addition or git-style initialisation
- Skill handles four relationship modes when an existing todo is found: replace, extend, sibling, new sections
- Sibling mode renumbers existing top-level tasks one level deeper and adds new work as a peer top-level item — user confirms the full renumbered list before any files are written
- Task breakdown proposed and confirmed by user before files are written (step 5 gate)
- `_changelog` is not created by `todo-begin` — it is created as part of the execution workflow when work actually begins
- Skill has 8 steps: check context → relate to existing work → understand intent → understand solution direction → understand delivery shape → propose task breakdown → finalise documents → present and confirm
- Docs are written incrementally — stub files created in step 1, sections populated as each conversation step completes
- Collaboration principles block added: ask don't assume, one question at a time, play back before moving on, read the room, show the picture building, be mindful of time
- Playback moments added at transitions: steps 3→4, 4→5, 5→6
- Step 6 asks the user how they want to navigate the breakdown (logical map, user/persona, feature set, journey/scenario) before proposing tasks
- Step 5 surfaces delivery shape (milestones, phases, priorities) before task granularity — informs ordering in step 6
- Skill is a draft — further improvements planned

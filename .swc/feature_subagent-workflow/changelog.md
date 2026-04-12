# PR-5 Changelog — Claude Code subagent workflow

Append-only. One section per task. Records what was decided, changed, or learned during implementation.

---

## Session — progress skill path fix `2026-04-12`

- Fixed script path in `swc_workflow-progress` example command — was referencing `swc-workflow-progress` (hyphen) instead of `swc_workflow-progress` (underscore), causing the invocation example to point at a non-existent directory

## Session — swc_push end-to-end flow `2026-04-11`

- Rewrote `swc_push` to handle commit and push directly — previously stopped after updating docs and left git ops to the user
- PR comment now happens after the push, not before; draft shown for approval before posting

## Session — swc_push PR comment ordering fix `2026-04-11`

- Reordered `swc_push` flow: PR comment is now drafted and posted only after the user confirms ready to commit and push — previously it fired during doc prep before any git confirmation
- Draft is shown to the user for approval before posting; skill never posts without explicit confirmation

## Session — implement stage + orchestrator gate behavior `2026-04-11`

- Built `swc_workflow_deliver-implement` — new third stage in the deliver workflow; spawns a placeholder agent with work item identifier only, evaluates four exit criteria, reports pass/fail, returns control to orchestrator
- Updated `swc_workflow-orchestrator` stage gate: replaced silent auto-retry with an explicit user prompt listing unmet criteria and offering re-invoke / skip / stop
- Wired `implement` as third stage in `swc_workflow_deliver` workflow JSON; `on_complete` updated to "Ready for review"
- Workflow tested and confirmed behaving as expected

## Session — specs stage implementation + BA requirements methodology `2026-04-11`

- Implemented `swc_workflow_deliver-specs` — full stage replacing the previous stub; two-path routing (lightweight vs full) based on complexity signals
- Lightweight path: two questions (success condition + explicit failure cases), writes bullet-list `specs.md`; no personas, no EARS, no Gherkin — fast exit for simple work items
- Full path: personas → user journeys (happy/alternative/error) → EARS requirements (REQ-NN IDs, typed by pattern) → Gherkin acceptance scenarios (one per requirement) → validation rules (conditional); writes structured `specs.md` as TDD brief for implementation agent
- Default bias: AI-unsupervised implementation always signals full path unless work item is clearly trivial
- Created `hooks/ways/swc/planning/requirements/way.md` — BA requirements lens for planning conversations; fires as child of planning way; adds stakeholder, customer journey, scope-edge, NFR, assumptions, dependencies, and risks questions to Intent and Solution stages without adding a new stage
- Marked 1.4.2.2 in-progress (specs stage WIP — stage is built, not yet validated end-to-end)

---

## Session — deliver workflow requirements stage and entry point `2026-04-10`

- Implemented `swc_workflow_deliver-requirements` — full stage with SWC/codebase context load, intent clarification conversation, intent confirmation gate before solution direction, and `requirements.md` output to `.swc/<folder>/workitems/<N>/`
- Updated `swc_workflow_deliver` entry point — work item status check (in-progress recap, not-started+docs info, done handling), new task scenario, explicit Skill tool invocation for orchestrator, `Skill` added to allowed-tools
- Fixed orchestrator bypass — deliver skill now explicitly guards against inline stage execution
- Added stub exit criteria to `swc_workflow_deliver-specs` so orchestrator gate check has something to verify
- Refactored `swc_resolver` → `swc_lookup` (user-led); deliver and requirements skills updated to match
- Documented deliver workflow stage design in `notes.md` — stage split rationale, per-task doc path, new task scenario ownership
- Added workload items 1.4.2.6 (status updates during delivery) and 1.4.5.3 (broader doc updates on acceptance)
- Marked 1.4.2.1 done — Gate 1 approach is addressed by the requirements stage context-loading and solution direction steps

---

## Session — swc_workflow_deliver skill scaffold `2026-04-09`

- Created `swc_workflow_deliver` orchestrator skill — resolves work item, confirms with user, delegates to `swc-workflow-orchestrator` with 2-stage deliver definition
- Created `swc_workflow_deliver-requirements` stub — stage 1, scope/constraints/edge-case clarification
- Created `swc_workflow_deliver-specs` stub — stage 2, test strategy and acceptance criteria
- Added workload item 1.4.2.0 to track skill wiring; marked done
- Added 1.2.12 to track `_meta.json` key rename (`workloads` → `workloadContexts`)

---

## Session — implementation workflow brief format `2026-04-09`

- Added brief format to notes.md — structure the implementation agent receives, extended from swc_execute to include prior context.md and review findings for multi-pass delivery
- Captured "Done when" vs spec relationship — spec supersedes prose criteria as the operative exit condition; implementation workflow exits when spec passes

---

## Task 1.4.1 — Resolve execution workflow risks `2026-04-09`

- R1 resolved: `swc_deliver` reads codebase context before Gate 1 so approach agreement is grounded in reality
- R2 resolved: quality loop escalates to user after second pass — user holds the exit condition, can accept as tech debt
- R3 resolved: implementation workflow treats context.md sections as a required checklist before agent can return
- R4 resolved: Gate 2 is a spec approval gate, not a test file gate — spec format agreed in architecture discussion; table of spec types by work item type added to notes
- R5 resolved: Gate 3 always includes reviewer findings from final pass; multi-pass handoff shows what was flagged vs resolved
- Added 1.2.11: resolve workload guard / skill conflict (hook fires on all Edit calls to workload.md including skill-mediated ones; deferred — aligns with planned CLI work)

---

## Workload restructure `2026-04-06`

- Reorganised workload from 15 flat top-level tasks into a 2-level hierarchy: **1. MAIN WORKFLOW** (sub-areas 1.1–1.4) and **2. Advanced improvements**
- Items regrouped thematically — planning visualisation now under 1.3, story mapping / ways hygiene / MCP service demoted to section 2
- Task number references updated in notes.md (task 10.2 → 1.3.3.2) and architecture.md (tasks 3.1, 12.1, 13.1, 9.1 → new numbers)
- Task 12.1 (naming convention alignment) marked as resolved — decision absorbed into completed items under 1.1.x

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

## Task 3.2 — Explicit git state check in plan-context `2026-03-29`

- Added explicit git repo state check as the first step of `todo-plan-context`
- Surfaces branch, remote, and open PRs in a single exchange before any planning begins
- Motivation: avoids assuming PR number from context; user confirms before proceeding
- Tasks 12.1 and 13.1 added to the breakdown at the same time (branch naming convention, new-project setup)

## Refactor — Migrate todo-begin from agent to main-session skill `2026-03-29`

- `agents/todo-plan.md` retired; `todo-begin` rewritten to invoke `todo-plan-*` skills in sequence directly from the main session
- Root cause: agents are for autonomous work; interactive planning conversations require back-and-forth that belongs in the main session. The agent approach caused double-spawning bugs and split context
- All six `plan-*` skills gained `## Exit criteria` sections (done-when + return control) so `todo-begin` owns the sequence and each skill is decoupled
- Planning way updated: "compress" means fewer questions, not fewer outputs
- Acceptance criteria checklist moved into `plan-finalise`

## Refactor — Rename plan-* skills to todo-plan-* `2026-03-29`

- All six planning phase skills renamed: `plan-context`, `plan-intent`, `plan-solution`, `plan-delivery`, `plan-breakdown`, `plan-finalise` → `todo-plan-*`
- Aligns the planning skill family with the `todo-` prefix convention established in Task 2.1
- Old `plan-*` directories removed after rename confirmed

## Task 10.3 — Build abstracted workflow orchestrator skill `2026-03-30`

- Built `todo-workflow-orchestrator` skill: generic engine driven by a JSON workflow definition passed by the calling skill
- Manages stage sequencing, banner emission, and gate enforcement — extracted from `todo-begin` to avoid duplicating this logic in every multi-stage workflow
- `todo-begin` refactored to a thin entry point that delegates to the orchestrator
- Test scaffolding added: `todo-test-workflow` (two-stage greeting sequence), `todo-test-stage1` (greet), `todo-test-stage2` (goodbye) — validates the orchestrator pattern end-to-end
- Workflow definition format: JSON with `title`, `stages` array (each with `name` and `skill`), and `active_stage` index

## Session — Rename resolver and consolidate branch/folder logic `2026-03-31`

- Renamed `swc_workload-resolver` → `swc_resolver` — shorter name reflects its broader role as the single source of truth for branch→folder naming
- Added main/master branch warning to `swc_resolver`: prompts user to create a feature branch, but allows continuing on main if they choose
- Added `--create` mode to `swc_resolver`: handles branch detection, folder naming, meta.json update, and folder creation — used by `swc_plan-context`
- Refactored `swc_plan-context` to delegate branch/folder setup to `swc_resolver --create`; removes duplicated naming logic
- Updated `swc_workload` reference and `settings.local.json` permission entry

## Session — Relocate .swc to repo root and add workload display skills `2026-03-31`

- Moved workload tracking from `.claude/.swc/` to `.swc/` at repo root — avoids path confusion between the repo and the config dir since they are the same thing
- Renamed `meta.json` → `_meta.json` to distinguish it from workload content files
- Added `swc_workload-resolver` skill: canonical way to resolve the active workload path from branch name via `_meta.json`, with folder-scan fallback and user confirmation
- Added `swc_workload` skill: displays work items with visual status symbols (✔/▣/□) and strikethrough for done items
- Removed `swc_list` skill (superseded by `swc_workload`)
- Updated `.gitignore` allowlist, all skill path references, and both swc ways to use new `.swc/` root location
- Task 7.1 marked done

## Session — SWC namespace reorganisation `2026-03-31`

- Skills renamed from `todo-*` to `swc_*` — underscore separates namespace from name, hyphens join words within (e.g. `swc_plan-context`, `swc_report-notes`)
- Ways moved from `hooks/ways/meta/swc/` to `hooks/ways/swc/` — swc now has its own top-level domain
- Hook moved from `hooks/swc-workload-guard.sh` to `hooks/swc/workload-guard.sh`; `settings.json` updated
- Tracking migrated from `todo-pr-*.md` files to `.swc/` workload structure; old files untracked from git
- `.claude/.gitignore` deleted (redundant given root allowlist); root `.gitignore` updated with `.swc/` exemptions
- Notes and plan docs updated to reflect new skill names, ways paths, and workload file locations

## Session — Extract swc_init and add non-git-repo support to resolver `2026-03-31`

- Extracted stub-file creation from `swc_plan-context` into a new `swc_init` skill — single responsibility: write the five stub docs, print nothing, return the folder path
- `swc_plan-context` step 3 now delegates to `swc_init` rather than creating files inline
- `swc_resolver` gained non-git-repo awareness: checks `git rev-parse --is-inside-work-tree` before any branch logic; if not a repo, offers to `git init` or fall back to directory name as the identifier
- Branch recommendation prompt redesigned: open text entry (name → checkout, Enter → stay) replaces binary y/n; also fires after `git init`, not just on main/master
- Silent fallback for missing `_meta.json` (removed the "not found" notice)
- Folder-creation responsibility moved from `swc_resolver` to `swc_init` — resolver now stops after writing `_meta.json`
- Closes task 13.1 (new-project setup now handled end-to-end by resolver + init pair)

## Session — Rename swc_begin and swc_plan-* to swc_workflow_plan-* `2026-03-31`

- `swc_begin` renamed to `swc_workflow_plan` — entry point name now reflects what it does (start the planning workflow)
- All six planning phase skills renamed: `swc_plan-context/intent/solution/delivery/breakdown/finalise` → `swc_workflow_plan-*` — groups them clearly under the planning workflow
- `swc_update` renamed to `swc_workload-update` — aligns with the `swc_workload` family naming
- `planning/way.md` vocabulary and pattern updated to reference `swc-workflow-plan`
- `workload-guard/way.md` updated: all references to `swc-update` → `swc-workload-update`
- `swc_init/SKILL.md` and `swc_resolver/SKILL.md` references updated to new names
- Old `swc_begin`, `swc_plan-*`, and `swc_update` skill directories removed

## Session — swc_workflow_plan-intent scope boundary `2026-03-31`

- Added explicit scope boundary section to `swc_workflow_plan-intent` — skill must not drift into tech decisions
- Deflection script added: acknowledge tech mentions, park them in `notes.md` under `## Parked (intent phase)`, then steer back to intent
- Reworded constraints question from "things we must not break" (invites tech thinking) to "non-negotiable outcomes — things that must hold true regardless of approach"
- Motivation: observed on multiple occasions that the skill was pulling into solution territory during intent phase; intent and solution are separate conversations

## Session — add swc_push skill `2026-03-31`

- Created `swc_push` skill (renamed from `swc_ship`): mid-session content prep before commit/push
- Flow: summarise changes → confirm with user → update changelog + docs → offer PR comment → confirm ready
- PR comment step: checks for open PR via `gh pr view`, drafts 3–5 bullet summary, posts on confirmation
- Skill stops before git ops — commit/push is user's next step
- Distinction from `/ship`: push = content hygiene mid-PR; ship = git delivery at merge moment

## Session — solution design stage `2026-04-13`

- Built `swc_workflow_deliver-solution-design` skill: new fourth stage in the deliver workflow, inserted between specs and implement
- Stage reads requirements + specs, thinks forward through what the agent will encounter, surfaces specific technical questions or challenges for this work item only
- Offers a gentle depth invite but defaults to brief if the user is satisfied with the directional approach
- Writes `solution.md` to `workitems/<N>/` — even when no blockers found, so the brief reference is consistent
- Updated `swc_workflow_deliver` workflow JSON: requirements → specs → solution-design → implement
- Updated `swc_workflow_deliver-implement`: `solution.md` added as a brief reference doc (by path, not inlined); phase number corrected to fourth
- Updated `notes.md`: deliver stage split documentation expanded to include solution-design with rationale
- Rationale: the agent is fully autonomous once spawned; unresolved implementation questions become silent deviations or a stuck agent; this stage seals the brief with hard guesses already answered

## Session — planning workflow UX improvements `2026-04-02`

- `swc_workflow_plan`: added step 0 — offers a workflow summary and confirms the user wants to proceed before starting; guards against false-positive skill triggers
- Stage summary is derived dynamically from the JSON config stages array, not hardcoded
- `swc_workflow_plan-intent`: removed the separate depth-check step ("quick vs thorough"); depth is now read from the user's answers throughout the conversation
- Playback in intent stage now doubles as the depth offer: "does that capture it, or is there anything you'd like to clarify or go deeper on?"
- `hooks/ways/swc/planning/way.md`: "be mindful of their time" rewritten — depth is a dial, not a switch; every stage still runs and must satisfy exit criteria; only check in explicitly if user intent is unclear

## Task 10.1 — Workflow progress banner `2026-03-30`

- Built `todo-workflow-progress` skill: emits a visual banner showing all stages with the active stage highlighted
- Wired into `todo-begin` — banner fires at the start of each planning stage
- All "step" references standardised to "stage" across `todo-begin` and the plan-* skills to align with gate semantics
- Tasks 10.2, 10.3, and 15 (MCP service) added to the breakdown with supporting notes

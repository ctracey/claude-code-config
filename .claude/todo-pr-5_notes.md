# PR-5 Notes — Claude Code subagent workflow

## Doc purpose

| Doc | Purpose |
|-----|---------|
| `_plan` | What and why — goals, features, scope, intent. Written upfront. |
| `_notes` | How — conventions, agreements, decisions that apply across tasks. Stable reference. |
| `_changelog` | What happened — append-only per-task record of decisions, changes, and learnings. |
| `_architecture` | Tech stack, folder structure, hard constraints. |

Read `_notes` to understand the rules. Read `_changelog` to understand why they are the way they are.

## Skill naming convention

All todo-related skills use noun-first, `todo-` prefix naming to:
- Align with the tracking file naming (`todo-pr-N.md`)
- Distinguish clearly from Claude Code's native `Task*` tools

| Skill | Purpose |
|-------|---------|
| `todo-list` | Display the task list visually |
| `todo-report-plan` | Concise summary of the `_plan` doc, with invitation to ask for detail |
| `todo-report-notes` | Concise summary of the `_notes` doc, with invitation to ask for detail |
| `todo-changelog` | Recent changelog entries, with invitation to ask for more |
| `todo-update` | Update task status in the file |
| `todo-add` | Add a new task to the list |
| `todo-begin` | Scaffold a new todo + plan + architecture set |
| `todo-execute` | Spawn an implementation subagent for a task |

## Skill invocation

Skills support both explicit and implicit invocation:
- **Explicit** — user types `/skill-name` or `/skill-name [args]`
- **Implicit** — Claude infers intent from the description field in the skill frontmatter

Both modes should be supported for all skills in this workflow.

### todo-execute usage

```
/todo-execute                  # next unchecked task from active todo-pr-N.md
/todo-execute 3                # specific task number
/todo-execute pr-5 task-3      # explicit PR and task scope
```

Implicit triggers: "work on task 3", "start the next task", "execute task 2"

## Task status convention

Parent task status rolls up from subtasks:
- Any subtask in progress or done (but not all done) → parent is `[-]`
- All subtasks done → parent is `[x]`
- No subtasks started → parent is `[ ]`

Markers: `[ ]` not started, `[-]` in progress, `[x]` done

## todo-list display format

Visual symbols used by the `todo-list` skill:

| Status | Symbol | Text treatment |
|--------|--------|----------------|
| Done | `✔` | Unicode combining strikethrough (U+0336 after every character) |
| In progress | `▣` | Plain text |
| Not started | `□` | Plain text |

Subtasks indented with two spaces. Output as plain text — no markdown code block.

## Changelog entry format

```markdown
## Task N.M — Description `YYYY-MM-DD HH:MM`

- Decision or change made
- Why it was made
```

Entries are in **chronological order** — appended as work happens, not sorted by task number.

## File naming convention

- Hyphen couples name parts (tighter binding): `todo-pr-5`
- Underscore separates a doctype qualifier (looser binding): `todo-pr-5_plan`
- Always lowercase

## Available agents and relation to this plan

| Agent | Description | Relation to this plan | Author |
|---|---|---|---|
| `workflow-orchestrator` | Coordinates ADR-driven workflow, guides debate→ADR→branch→implement→PR | Overlaps with the main session orchestrator role — but ADR-focused, not todo-task-focused | Aaron Bockelie |
| `task-planner` | Plans work using branches and TodoWrite, breaks down complex work | Overlaps with todo-begin (3.1) — scaffolding a task breakdown from a plan | Aaron Bockelie |
| `code-reviewer` | Reviews code for quality, SOLID, requirement traceability | Is the review subagent (6.2) — just needs context wiring | Aaron Bockelie |
| `Plan` | Designs implementation strategy, identifies critical files | Could feed into implementation subagent briefs, or replace parts of todo-begin | Claude Code built-in |
| `system-architect` | Drafts ADRs, never implements | Not directly in scope unless ADR gates are added to the workflow | Aaron Bockelie |

> **Note on ADRs:** Aaron's ADR technique (`workflow-orchestrator`, `system-architect`) is an **architectural lead workflow** — it captures decisions, trade-offs, and rationale for future reference. This PR-5 workflow is a **task/outcome driven workflow** — it cares about getting work done, reviewed, and shipped one task at a time. The two can coexist: an ADR could precede and inform a todo breakdown, but ADR authorship is not part of this workflow's loop.

## task-planner integration with todo-begin (3.1)

`task-planner` is a good candidate to drive the breakdown step inside `todo-begin`. The proposed split:

1. `todo-begin` receives the feature description from the user
2. Spawns `task-planner` as a subagent to reason about task breakdown, dependencies, and branch strategy
3. Main session translates `task-planner`'s output into the persistent `todo-pr-N.md` format (with `_plan`, `_notes`, `_architecture` stubs)

Key mismatch to resolve: `task-planner` thinks in ephemeral TodoWrite + git branches; `todo-begin` needs persistent markdown files. The translation step is the main session's responsibility, not the subagent's.

`task-planner` should receive the `_plan` doc (goal, features, scope) as context so it breaks down work that matches the stated intent.

## Acceptance criteria for planning

The planning conversation is complete when the user can close all sessions, return the next day, open a new session, and — using only the docs — pick up delivery with confidence. No re-explaining context, no re-describing the goal, no re-making decisions already taken.

## Documents are the only briefing the implementer gets

The planning conversation happens in its own session. The implementation subagent — and any future session — will not have access to that conversation. The docs are the only thing they will see.

Every decision that influenced the work must be captured: what was decided, and why. Every constraint, intent, and direction agreed during planning. Not a transcript — but complete enough that someone who was not in the room can read the docs and do the work correctly.

If it shaped the direction, it belongs in the docs. Write to them throughout the conversation as agreements are reached. The final review is just confirming nothing was missed.

## Todo file tracking in git

Todo files (`todo-pr-N*.md`) are first-class documentation — equivalent to specs, plans, and changelogs. They are tracked in git alongside the PR branch they describe.

- Tracked: `todo-pr-N.md`, `todo-pr-N_plan.md`, `todo-pr-N_notes.md`, `todo-pr-N_changelog.md`, `todo-pr-N_architecture.md`
- `.gitignore` allowlist pattern: un-ignore `.claude/` directory, then `!.claude/todo-pr-*.md`
- Motivation: these files give reviewers full context on intent, decisions, and task breakdown — valuable PR documentation that should live in version control

## CLI tool for todo management

The todo skills (todo-list, todo-update, todo-add, etc.) could be backed by a small CLI tool rather than Claude reading/writing markdown directly. Benefits:

- Structured reads/writes — no risk of Claude mangling the file format
- Skills become thin wrappers: invoke CLI, present output
- CLI can be tested independently of Claude
- Could expose a `todo` command: `todo list`, `todo add`, `todo done 2.3`, `todo status`

Candidate approaches: a standalone shell script, a small Node/Python CLI, or a Bash wrapper. The CLI reads and writes `todo-pr-N.md` files; the skills handle presentation and user interaction.

Decision not yet made — captured here as a direction worth exploring when building todo-add (2.7). The `todo-update` skill was built without a CLI backing and works reliably for direct markdown edits — revisit for todo-add if complexity warrants it.

## Packaging this workflow

The skills, ways, hooks, and agents that make up this workflow should be distributable as a unit. Options:

| Approach | Description |
|---|---|
| **Plugin** | Claude Code plugin bundling skills + ways + agents as a named package |
| **Dotfiles repo** | Fork of `~/.claude` with workflow pre-installed — user clones to get everything |
| **Install script** | Shell script that copies skills/ways/agents into `~/.claude` |
| **Project scaffold** | `todo-begin` skill writes the workflow files into `.claude/` at project setup time |

The plugin approach is cleanest for distribution (single install, versioned), but depends on Claude Code's plugin support. The install script is the most portable today.

Decision not yet made — worth revisiting when the workflow is stable enough to package.

## Ways and fresh agent sessions

Each subagent gets its own session, which means ways fire fresh for every subagent spawn. This is a feature, not a coincidence — it ensures that project preferences, code style, and workflow guidance are always present regardless of how far into a long main session we are.

Practical implications:

- **Implementation subagent** — ways fire on first relevant action (e.g. editing a file triggers `meta/tracking`, running a commit triggers `delivery/commits`). The subagent always gets clean, un-drifted guidance.
- **Review subagent** — same: `code/quality`, `code/security`, and other review-relevant ways fire fresh when it starts examining code.
- **Main session** — ways fire once per session. In a long session, guidance injected early may be far back in context. Spawning a subagent to handle a bounded task (rather than doing it inline) keeps the main session clean and ways fresh where they matter.

This is a reason to prefer subagents for implementation and review work even when the main session *could* do it — fresh ways are a form of context hygiene.

## Workflow components inventory

**Skills** (built by this workflow)

| Skill | Status | Purpose |
|---|---|---|
| `todo-list` | ✔ done | Display task list with visual symbols |
| `todo-report-plan` | ✔ done | Summarise the `_plan` doc |
| `todo-report-notes` | ✔ done | Summarise the `_notes` doc |
| `todo-changelog` | ✔ done | Show recent changelog entries |
| `todo-report` | ✔ done | Full status report (plan + list + notes) |
| `todo-execute` | ✔ done | Spawn implementation subagent for a task |
| `todo-update` | ✔ done | Update task status in file, with parent rollup |
| `todo-add` | □ not built | Add a new task |
| `todo-begin` | ◐ draft | Scaffold new todo + plan + architecture |
| `user-handoff` | □ not built | Structured handoff before commit |
| `implementation-workflow` | □ not built | Governs the impl subagent step-by-step |
| `review-subagent` | □ not built | Review findings format + skill |

**Ways** (pre-existing, relevant to this workflow)

| Way | Trigger | Relevance |
|---|---|---|
| `meta/subagents` | keyword: subagent/delegate/spawn | How to invoke subagents — directly used by todo-execute |
| `meta/tracking` | file edit: `.claude/todo-*.md` | Defines the tracking file format this workflow builds on |
| `meta/todos` | context threshold >75% | Prompts task capture before compaction |

**Agents** (pre-existing, wired into this workflow)

| Agent | Role in this workflow |
|---|---|
| `code-reviewer` | Is the review subagent (6.2) — needs context wiring |
| `task-planner` | Candidate for breakdown step in `todo-begin` (3.1) |
| `Plan` (built-in) | Candidate for architecture analysis in `todo-begin` |

**Hooks** (planned, not yet built)

| Hook | Purpose |
|---|---|
| Pre-commit / pre-push | Confirm user is ready before git operations |

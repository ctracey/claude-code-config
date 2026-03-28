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
| `todo-plan` | Concise summary of the `_plan` doc, with invitation to ask for detail |
| `todo-notes` | Concise summary of the `_notes` doc, with invitation to ask for detail |
| `todo-changelog` | Recent changelog entries, with invitation to ask for more |
| `todo-update` | Update task status in the file |
| `todo-add` | Add a new task to the list |
| `todo-new` | Scaffold a new todo + plan + architecture set |
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
| `task-planner` | Plans work using branches and TodoWrite, breaks down complex work | Overlaps with todo-new (3.1) — scaffolding a task breakdown from a plan | Aaron Bockelie |
| `code-reviewer` | Reviews code for quality, SOLID, requirement traceability | Is the review subagent (6.2) — just needs context wiring | Aaron Bockelie |
| `Plan` | Designs implementation strategy, identifies critical files | Could feed into implementation subagent briefs, or replace parts of todo-new | Claude Code built-in |
| `system-architect` | Drafts ADRs, never implements | Not directly in scope unless ADR gates are added to the workflow | Aaron Bockelie |

## Todo file tracking in git

Todo files (`todo-pr-N*.md`) are first-class documentation — equivalent to specs, plans, and changelogs. They are tracked in git alongside the PR branch they describe.

- Tracked: `todo-pr-N.md`, `todo-pr-N_plan.md`, `todo-pr-N_notes.md`, `todo-pr-N_changelog.md`, `todo-pr-N_architecture.md`
- `.gitignore` allowlist pattern: un-ignore `.claude/` directory, then `!.claude/todo-pr-*.md`
- Motivation: these files give reviewers full context on intent, decisions, and task breakdown — valuable PR documentation that should live in version control

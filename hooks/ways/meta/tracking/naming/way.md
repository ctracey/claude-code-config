---
description: Naming convention for tracking files — doctype suffix pattern
vocabulary: doctype plan architecture notes naming convention suffix tracking file
threshold: 2.0
files: \.claude/todo-.*\.md$
scope: agent, subagent
---
# Tracking File Naming Convention

Tracking files follow a consistent naming pattern:

```
.claude/
├── todo-pr-N.md               # PR task list
├── todo-pr-N_plan.md          # plan doc for PR N
├── todo-pr-N_architecture.md  # architecture doc for PR N
├── todo-pr-N_notes.md         # conventions and agreements for PR N
├── todo-pr-N_changelog.md     # append-only per-task record for PR N
├── todo-adr-NNN.md            # ADR task list
├── todo-adr-NNN_plan.md       # plan doc for ADR NNN
├── todo-issue-N.md            # issue task list
```

## The Rule

**Hyphen** couples items that belong to the same name: `todo-pr-5`

**Underscore** separates a doctype qualifier from the name: `todo-pr-5_plan`

This means the underscore is always a boundary — everything before it is the identity, everything after it is the type of document.

## Doctypes

| Suffix | Purpose | Used by |
|--------|---------|---------|
| *(none)* | Task list — progress tracking | Main session |
| `_plan` | **What and why** — goals, features, intent, out of scope | Main session → subagents |
| `_architecture` | Tech stack, folder structure, hard constraints | Main session → subagents |
| `_notes` | **Conventions and agreements** — naming, format, decisions that apply across tasks. Stable reference. | Any actor |
| `_changelog` | **What happened** — append-only per-task record of decisions, changes, and learnings. | Any actor |

**`_plan` captures upfront intent.** What are we building and why?

**`_notes` is a stable reference.** Conventions agreed mid-session that must survive across sessions. Read this to understand the rules.

**`_changelog` is append-only.** One section per task, recording what was decided, changed, or learned during implementation. Read this to understand why the rules are the way they are.

Changelog entry format:
```markdown
## Task N.M — Description `YYYY-MM-DD HH:MM`

- Decision or change made
- Why it was made
```

Entries are in **chronological order** — appended as work happens, not sorted by task number.

When a new session picks up mid-work, read `_notes` first, then skim `_changelog` for recent context.

## Always lowercase

`todo-pr-5.md` not `todo-PR-5.md`

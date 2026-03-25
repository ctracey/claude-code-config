---
description: Naming convention for tracking files — doctype suffix pattern
vocabulary: doctype plan architecture naming convention suffix tracking file
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
├── todo-adr-NNN.md            # ADR task list
├── todo-adr-NNN_plan.md       # plan doc for ADR NNN
├── todo-issue-N.md            # issue task list
```

## The Rule

**Hyphen** couples items that belong to the same name: `todo-pr-5`

**Underscore** separates a doctype qualifier from the name: `todo-pr-5_plan`

This means the underscore is always a boundary — everything before it is the identity, everything after it is the type of document.

## Doctypes

| Suffix | Contents |
|--------|----------|
| *(none)* | Task list — the primary tracking file |
| `_plan` | Goals, features, product intent, out of scope |
| `_architecture` | Tech stack, folder structure, hard constraints |

## Always lowercase

`todo-pr-5.md` not `todo-PR-5.md`

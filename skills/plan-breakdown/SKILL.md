---
name: plan-breakdown
description: Propose and confirm the task breakdown for a piece of work. Fifth phase of the planning conversation. Use when creating a task list, or when invoked via /plan-breakdown.
allowed-tools: Read, Write, Edit, Glob
---

# Plan Breakdown

Create the task list. Always confirm before writing.

## Steps

### 1. Ask how to navigate

> "How do you want to organise the tasks — do you have a logical map in mind, or would you prefer to walk through it by user, feature set, or journey/scenario?"

| Approach | What it means |
|---|---|
| Logical / technical map | Tasks follow the system structure — components, layers, modules |
| User / persona | Tasks grouped by who benefits |
| Feature set | Tasks grouped by capability area |
| Journey / scenario | Tasks follow an end-to-end flow |

The user may mix these. Follow their lead — don't impose structure.

### 2. Draft the task list

With the delivery shape and navigation approach in mind, draft the full task list. Present as plain text using the `todo-list` visual format:

```
□ 1. First task
  □ 1.1. Subtask
  □ 1.2. Subtask
□ 2. Second task
  □ 2.1. Subtask
```

For **Sibling** mode, show the full renumbered list — existing tasks nested under `1.` and new work under `2.` — before any files change.

### 3. Confirm

> "Does this numbering and breakdown look right? Say yes to write the files, or tell me what to adjust."

Wait for explicit confirmation. Iterate if requested. Do not write until confirmed.

### 4. Capture

Write the confirmed task list to `todo-pr-N.md`:

```markdown
# PR-N Task Breakdown — [work title]

## Tasks

- [ ] **1. [First task]**
  - [ ] 1.1. [Subtask]
  - [ ] 1.2. [Subtask]

- [ ] **2. [Second task]**
  - [ ] 2.1. [Subtask]
```

Task granularity: small and specific, with a clear "done when" implied by each name. More than ~5 subtasks under one parent → consider splitting the parent.

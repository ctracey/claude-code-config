---
name: todo-update
description: Update the status of a task or subtask in the active todo file. Use when the user says "mark X done", "mark X as done", "start task X", "complete task X", "task X is done", or invokes /todo-update.
allowed-tools: Read, Edit, Glob
---

# Todo Update

Update a task or subtask status in the active `todo-pr-N.md` file, then roll up the parent status.

## Arguments

- `/todo-update <task> done` — mark a task done (`[x]`)
- `/todo-update <task> in-progress` — mark a task in progress (`[-]`)
- `/todo-update <task> reset` — mark a task not started (`[ ]`)
- Task can be a number (`2.6`), a description match, or implied from context ("the current task", "next task")

## Steps

### 1. Resolve the file

- Find the most recently modified `.claude/todo-pr-*.md` in `.claude/`
- If a PR number is explicit, use `.claude/todo-pr-N.md`

### 2. Resolve the target task

- Match by task number (e.g. `2.6`) or description keyword
- If ambiguous, confirm with the user before editing

### 3. Apply the status change

Update the checkbox on the matched line:
- `done` → `[x]`
- `in-progress` → `[-]`
- `reset` → `[ ]`

### 4. Roll up the parent status

After updating the subtask, re-evaluate the parent task status:

| Subtask state | Parent becomes |
|---|---|
| All subtasks `[x]` | `[x]` |
| Any subtask `[-]` or `[x]`, but not all `[x]` | `[-]` |
| All subtasks `[ ]` | `[ ]` |

Update the parent line if its current marker does not match the rolled-up result.

If the parent has no subtasks, update it directly — no rollup needed.

### 5. Confirm

Output a single confirmation line, e.g.:

```
✔ Marked 2.6 done. Parent task 2 updated to [-] (1.3 still outstanding).
```

No other output.

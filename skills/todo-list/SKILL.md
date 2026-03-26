---
name: todo-list
description: Display the active todo task list in a visual format with status symbols. Use when the user says "show tasks", "show me the task list", "what tasks are left", or invokes /todo-list.
allowed-tools: Read, Glob
---

# Todo List

Read the active todo file and display tasks using visual status symbols.

## Arguments

- `/todo-list` — display tasks from the active `todo-pr-*.md`
- `/todo-list pr-N` — display tasks from a specific PR

## Steps

### 1. Resolve the file

- If PR number supplied, read `.claude/todo-pr-N.md`
- Otherwise, find the most recently modified `.claude/todo-pr-*.md` in `.claude/`

### 2. Parse tasks

Read every task line — both parent tasks and subtasks. Preserve the hierarchy.

Status is determined by the checkbox:
- `[x]` → done
- `[-]` → in progress
- `[ ]` → not started

### 3. Output the list

For each task, apply the symbol and formatting below. Output as plain text (no markdown code block, no backticks).

| Status | Symbol | Text treatment |
|--------|--------|----------------|
| Done | `✔` | Apply Unicode combining strikethrough (U+0336 after every character) |
| In progress | `▣` | Plain text |
| Not started | `□` | Plain text |

Indent subtasks with two spaces.

**Example output:**

TODO LIST
✔ 1̶.̶ ̶P̶a̶r̶e̶n̶t̶ ̶t̶a̶s̶k̶ ̶o̶n̶e̶
  ✔ 1̶.̶1̶.̶ ̶C̶o̶m̶p̶l̶e̶t̶e̶d̶ ̶s̶u̶b̶t̶a̶s̶k̶
  ▣ 1.2. In progress subtask
  □ 1.3. Not started subtask
▣ 2. Parent task two
  □ 2.1. Not started subtask
□ 3. Parent task three
  □ 3.1. Not started subtask

Output nothing else — no preamble, no trailing summary.

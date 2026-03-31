---
name: swc_workload
description: Display the active workload in a visual format with status symbols. Use when the user says "show tasks", "show me the task list", "show workload", "what work items are left", or invokes /swc-workload.
allowed-tools: Read, Glob, Bash
---

# SWC Workload

Read the active workload file and display work items using visual status symbols.

## Arguments

- `/swc-workload` — display work items from the active workload
- `/swc-workload <branch>` — display work items for a specific branch

## Steps

### 1. Resolve the active workload

Follow the `swc_resolver` skill to get the path to `workload.md`. If resolution fails or the user declines, stop.

Read the resolved file.

### 2. Parse work items

Read every work item line — both parent items and sub-items. Preserve the hierarchy.

Status is determined by the checkbox:
- `[x]` → done
- `[-]` → in progress
- `[ ]` → not started

### 3. Output the list

For each work item, apply the symbol and formatting below. Output as plain text (no markdown code block, no backticks).

| Status | Symbol | Text treatment |
|--------|--------|----------------|
| Done | `✔` | Apply Unicode combining strikethrough (U+0336 after every character) |
| In progress | `▣` | Plain text |
| Not started | `□` | Plain text |

Indent sub-items with two spaces.

**Example output:**

WORKLOAD  .swc/<branch-subfolder>/workload.md
✔ 1̶.̶ ̶P̶a̶r̶e̶n̶t̶ ̶w̶o̶r̶k̶ ̶i̶t̶e̶m̶ ̶o̶n̶e̶
  ✔ 1̶.̶1̶.̶ ̶C̶o̶m̶p̶l̶e̶t̶e̶d̶ ̶s̶u̶b̶-̶i̶t̶e̶m̶
  ▣ 1.2. In progress sub-item
  □ 1.3. Not started sub-item
▣ 2. Parent work item two
  □ 2.1. Not started sub-item
□ 3. Parent work item three
  □ 3.1. Not started sub-item

Output nothing else — no preamble, no trailing summary.

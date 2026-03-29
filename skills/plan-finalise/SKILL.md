---
name: plan-finalise
description: Finalise the planning docs — fill any remaining gaps, run todo-report, and confirm ready to start. Final phase of the planning conversation. Use at the end of a planning session or when invoked via /plan-finalise.
allowed-tools: Read, Write, Edit, Glob
---

# Plan Finalise

The docs have been built throughout the conversation. This step fills any remaining gaps and confirms everything is complete.

## Steps

### 1. Review for completeness

Check each doc:

- `todo-pr-N.md` — task list present and confirmed?
- `_plan.md` — Goal/Why, Features, Out of scope, Delivery shape?
- `_architecture.md` — Tech stack, Folder structure, Constraints?
- `_notes.md` — Doc purpose table, Solution decisions, Open questions, Deferred decisions?

Fill any missing sections. For **Extend**, **Sibling**, and **New sections** modes, edit rather than overwrite.

### 2. Run todo-report

Invoke `todo-report` to surface the summary view.

### 3. Close

> "The docs are up to date — here's the summary. If anything looks off, open the files directly:
> - `.claude/todo-pr-N_plan.md`
> - `.claude/todo-pr-N_architecture.md`
> - `.claude/todo-pr-N_notes.md`
>
> Ready to start, or anything to adjust?"

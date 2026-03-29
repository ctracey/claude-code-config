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

**Acceptance criteria — the docs are complete when a future session could answer all of these without asking the user again:**

- What we're building and why
- Who it's for and what they need
- What's in scope and what's explicitly out
- The approach agreed for the solution
- The delivery shape and priorities
- The task breakdown and what to start with
- What's been decided, what's an open question, and what's intentionally deferred

If any of these would require re-asking, the docs are not complete.

### 2. Play back the plan using todo-report

Run the `todo-report` skill now. This is the playback — present its full output to the user as the summary of what was planned.

`todo-report` covers: plan summary, task list, notes overview, and next step. This is the user's confirmation that the docs reflect what was agreed.

### 3. Close and hand back

After the report output, say:

> "That's the plan. If anything looks off, open the docs directly and we can adjust:
> - `.claude/todo-pr-N_plan.md`
> - `.claude/todo-pr-N_architecture.md`
> - `.claude/todo-pr-N_notes.md`
>
> Planning is done — head back to the main session and run `/todo-execute` to start the first task."

**Stop here.** Do not begin implementation. Do not write code. Return control to the main session.

## Exit criteria

**Done when:**
- All docs pass the acceptance criteria checklist
- `todo-report` playback has been presented
- User has explicitly confirmed the plan is correct

**Return control to `todo-begin`. Planning is complete.**

---
description: collaborative task delivery, incremental development with feedback loops, task breakdown and agreement, post-task handoff protocol
vocabulary: increment handoff breakdown approval iterative checklist agree acceptance sign-off loop task-by-task
threshold: 2.0
---
# Collaborative Delivery

We work one task at a time. After completing each task, stop and deliver a structured handoff — do not proceed to the next task until the user signals approval.

## Agree Before Starting

Before any implementation begins, agree on the task breakdown with the user. Confirm scope, order, and any open questions. Only start when the user says they are ready.

Once agreed, save the breakdown to `.claude/tasks.md` in the project directory. Use a simple numbered list with a one-line description per task. Update task status in place as work progresses (`[ ]` → `[x]`).

## Post-Task Handoff

After completing each task, provide this summary — then stop and wait:

| | |
|---|---|
| **Context** | How this task fits into the overall breakdown |
| **Changes** | Brief summary of what was done |
| **Tests** | Status and what was added or updated |
| **Docs** | What changed in README or elsewhere |
| **Dev server** | Where to find it (URL or command) |

Do not begin the next task. Do not suggest what comes next. Wait for the user to respond.

## Milestone Maturity Check

When a functional milestone is reached (feature works end-to-end, task group complete):

1. **Announce the milestone** — tell the user the feature is working and you're now doing a quick maturity pass
2. **Stage the changes** — `git add` the working state so there's a clear baseline (do not commit — commits require user approval)
3. **Assess code maturity** — review the working code for quality, structure, and adherence to project ways
4. **Capture refactoring actions** — save findings to `.claude/todo-refactor-<TASKNAME>.md` with specific, actionable items. Prefix refactor steps with `R` (R1, R2…) to distinguish from feature task numbers in the main task list
5. **Check in if the list is significant** — if there are more than a few items, or any items require structural changes, present the list to the user. Frame it as a trade-off: what's worth fixing now vs. accepting as tech debt to keep delivering. The user's appetite for this varies by project — let them decide
6. **Work through the refactoring** — keep this rapid and focused. Quick wins, not deep rewrites
7. **Handoff again** — deliver the post-task handoff summary so the user can review the cleaned-up state

This ensures working code ships clean, not just working. But it should not become a bottleneck — the maturity pass is a quick polish, not a second project.

## Branch and PR Pattern

One branch, one PR. Each completed task is a commit on that branch. The PR accumulates commits as tasks complete and represents the full change when done.

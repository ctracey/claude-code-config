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

Do not commit until the user explicitly confirms they are ready. They need the opportunity to test, review, and provide feedback before the work is locked in.

## Branch and PR Pattern

One branch, one PR. Each completed task is a commit on that branch. The PR accumulates commits as tasks complete and represents the full change when done.

Never commit directly to main. Create the branch before any implementation begins.

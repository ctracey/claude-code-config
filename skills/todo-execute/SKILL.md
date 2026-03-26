---
name: todo-execute
description: Execute a single task by spawning an implementation subagent with the task brief and reference docs. Use when the user says "work on task N", "start the next task", "execute task N", or invokes /todo-execute.
allowed-tools: Read, Agent
---

# Task Execution

Delegate a single task to a fresh implementation subagent. The main session never implements — it briefs, delegates, and receives.

## Arguments

- `/todo-execute` — pick up the next unchecked task from `.claude/todo-pr-N.md`
- `/todo-execute N` — execute a specific task number
- `/todo-execute pr-N task-M` — explicit PR and task scope

## Steps

### 1. Resolve scope

Determine the PR number and task:
- If PR number not supplied, find the active `todo-pr-*.md` in `.claude/` (most recently modified)
- If task number not supplied, find the first unchecked `- [ ]` item in the task list
- Confirm scope with the user before proceeding: "I'll work on **task M** from **PR N**: [task name]. Proceed?"

### 2. Load reference docs

Read in parallel:
- `.claude/todo-pr-N.md` — extract the full task entry (description, Context, Done when)
- `.claude/todo-pr-N_plan.md` — if it exists
- `.claude/todo-pr-N_architecture.md` — if it exists

### 3. Compose the task brief

Assemble a brief for the implementation subagent:

```
## Task brief

**Task:** [task name]
[task description]

**Context:** [from task entry]
**Done when:** [from task entry]

## Plan
[contents of todo-pr-N_plan.md, or "not provided"]

## Architecture
[contents of todo-pr-N_architecture.md, or "not provided"]
```

### 4. Spawn implementation subagent

```
Agent(
  subagent_type: "general-purpose",
  description: "Implement task: [task name]",
  prompt: "[task brief from step 3]

  Follow the implementation-workflow skill. Return a rich summary artifact when complete."
)
```

### 5. Receive and present the summary artifact

When the subagent returns:
- Present the summary artifact to the user
- Do not assess or edit it — that is the review subagent's job
- Ask: "Ready to run the review?"

## Key Principles

- One task at a time — never spawn multiple implementation subagents in parallel
- Always confirm scope before spawning — wrong task = wasted work
- Never implement anything directly — delegate only
- The summary artifact travels intact to the review step

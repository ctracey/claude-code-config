---
name: todo-begin
description: Begin a new piece of work — scaffold todo + plan + architecture from a feature description. Use when starting a new project, plan, or piece of work, or when the user says "begin new work", "start a new plan", "new project", "new piece of work", or invokes /todo-begin.
allowed-tools: Bash, Read, Write, Edit, Glob
---

# Todo Begin

Entry point for starting a new piece of work. Delegates the planning conversation to `todo-workflow-orchestrator`.

## Arguments

- `/todo-begin` — auto-detect PR number from current branch
- `/todo-begin N` — use PR number N

## Steps

Invoke `todo-workflow-orchestrator` with the following definition, passing the PR number argument (if given) as the `args` of the `context` stage:

```json
{
  "title": "planning",
  "stages": [
    { "name": "context",   "skill": "todo-plan-context",   "args": "<PR number or empty>" },
    { "name": "intent",    "skill": "todo-plan-intent",    "args": "" },
    { "name": "solution",  "skill": "todo-plan-solution",  "args": "" },
    { "name": "delivery",  "skill": "todo-plan-delivery",  "args": "" },
    { "name": "breakdown", "skill": "todo-plan-breakdown", "args": "" },
    { "name": "finalise",  "skill": "todo-plan-finalise",  "args": "" }
  ],
  "on_complete": "Planning complete. Run `/todo-execute` to begin the first task."
}
```

## Role boundary

**Plan. Do not implement.**

Implementation does not start until the user has explicitly confirmed the plan is correct — that confirmation is what `todo-plan-finalise` is waiting for.

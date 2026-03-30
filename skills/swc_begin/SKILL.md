---
name: swc_begin
description: Begin a new piece of work — scaffold workload + plan + architecture from a feature description. Use when starting a new project, plan, or piece of work, or when the user says "begin new work", "start a new plan", "new project", "new piece of work", or invokes /swc-begin.
allowed-tools: Bash, Read, Write, Edit, Glob
---

# SWC Begin

Entry point for starting a new piece of work. Delegates the planning conversation to `swc-workflow-orchestrator`.

## Steps

Invoke `swc-workflow-orchestrator` with the following definition:

```json
{
  "title": "planning",
  "stages": [
    { "name": "context",   "skill": "swc-plan-context",   "args": "" },
    { "name": "intent",    "skill": "swc-plan-intent",    "args": "" },
    { "name": "solution",  "skill": "swc-plan-solution",  "args": "" },
    { "name": "delivery",  "skill": "swc-plan-delivery",  "args": "" },
    { "name": "breakdown", "skill": "swc-plan-breakdown", "args": "" },
    { "name": "finalise",  "skill": "swc-plan-finalise",  "args": "" }
  ],
  "on_complete": "Planning complete. Run `/swc-execute` to begin the first work item."
}
```

## Role boundary

**Plan. Do not implement.**

Implementation does not start until the user has explicitly confirmed the plan is correct — that confirmation is what `swc-plan-finalise` is waiting for.

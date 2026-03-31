---
name: swc_workflow_plan
description: Begin a new piece of work — scaffold workload + plan + architecture from a feature description. Use when starting a new project, plan, or piece of work, or when the user says "begin new work", "start a new plan", "new project", "new piece of work", or invokes /swc-workflow-plan.
allowed-tools: Bash, Read, Write, Edit, Glob
---

# SWC Workflow Plan

Entry point for starting a new piece of work. Delegates the planning conversation to `swc-workflow-orchestrator`.

## Steps

Invoke `swc-workflow-orchestrator` with the following definition:

```json
{
  "title": "planning",
  "stages": [
    { "name": "context",   "skill": "swc-workflow-plan-context",   "args": "" },
    { "name": "intent",    "skill": "swc-workflow-plan-intent",    "args": "" },
    { "name": "solution",  "skill": "swc-workflow-plan-solution",  "args": "" },
    { "name": "delivery",  "skill": "swc-workflow-plan-delivery",  "args": "" },
    { "name": "breakdown", "skill": "swc-workflow-plan-breakdown", "args": "" },
    { "name": "finalise",  "skill": "swc-workflow-plan-finalise",  "args": "" }
  ],
  "on_complete": "Planning complete. Run `/swc-execute` to begin the first work item."
}
```

## Role boundary

**Plan. Do not implement.**

Implementation does not start until the user has explicitly confirmed the plan is correct — that confirmation is what `swc-workflow-plan-finalise` is waiting for.

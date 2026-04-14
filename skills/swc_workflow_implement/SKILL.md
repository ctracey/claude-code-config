---
name: swc_workflow_implement
description: Implementation workflow — drives an implementation agent through orient, implement, refine, and summarise stages. Entry point for the agent-side workflow. Use when an implementation agent receives a work item and needs to execute it, or when invoked via /swc-workflow-implement.
allowed-tools: Bash, Read, Write, Edit, Glob, Agent
---

# SWC Implementation Workflow

Entry point for the implementation agent. Reads the work item and folder from context, then delegates to `swc_workflow-orchestrator` with the four implementation stages.

## Context

The work item number, name, and workload folder path are passed in the agent prompt by `swc_workflow_deliver-implement`. These must be available before this skill runs.

## Steps

### 1. Confirm context

Extract from the calling prompt:
- Work item number (e.g. `1.4.4.1`)
- Work item name
- Workload folder path (e.g. `.swc/feature_subagent-workflow/`)

If any are missing, stop and surface the gap — do not proceed without a resolved work item.

### 2. Run the workflow

Invoke `swc_workflow-orchestrator` with the implementation stage definitions:

```json
{
  "title": "implement",
  "stages": [
    { "name": "orient",    "skill": "swc_workflow_implement-orient",    "args": "" },
    { "name": "implement", "skill": "swc_workflow_implement-implement",  "args": "" },
    { "name": "refine",    "skill": "swc_workflow_implement-refine",     "args": "" },
    { "name": "summarise", "skill": "swc_workflow_implement-summarise",  "args": "" }
  ],
  "on_complete": "Implementation workflow complete. Summary artifact written."
}
```

The work item number, name, and folder path are available to each stage skill via the calling context.

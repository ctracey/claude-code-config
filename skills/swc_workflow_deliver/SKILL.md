---
name: swc_workflow_deliver
description: Drive delivery of a work item — clarify requirements, define test strategy and acceptance criteria. Use when implementing a work item, delivering a task, starting implementation, "work on this", "let's build", "implement task N", or when invoked via /swc-workflow-deliver.
allowed-tools: Bash, Read, Write, Edit, Glob
---

# SWC Workflow Deliver

Entry point for delivering a work item. Delegates the delivery conversation to `swc-workflow-orchestrator`.

## Steps

### 0. Resolve the work item

If the user has specified a work item number or name, confirm it with them before proceeding.

If no work item was specified, resolve the active workload (via `swc_resolver`) and check for items with status `[-]` (in progress). If there is exactly one, confirm with the user:

> "I'll work on **[item number]: [item name]** — is that right?"

If there are multiple in-progress items, or none, ask the user which item they want to deliver.

Wait for confirmation before proceeding.

### 1. Confirm intent

Before starting, read the `stages` array from the JSON config in step 2. For each stage, render its `name` as a bullet with a one-line description of what that stage covers. Present to the user:

> "Ready to start the delivery workflow for **[item number]: [item name]**. It covers [N] stages:
> [generated bullets]
>
> Want to go ahead?"

If yes, proceed. If no, ask what they actually need and stop here.

### 2. Run the workflow

Invoke `swc-workflow-orchestrator` with the following definition:

```json
{
  "title": "deliver",
  "stages": [
    { "name": "requirements", "skill": "swc-workflow-deliver-requirements", "args": "" },
    { "name": "specs",        "skill": "swc-workflow-deliver-specs",        "args": "" }
  ],
  "on_complete": "Delivery workflow complete. Ready to implement."
}
```

## Role boundary

**Plan. Do not implement.**

Implementation does not start until the delivery workflow is complete and the user has confirmed they are ready to proceed.

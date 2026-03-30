---
name: swc_execute
description: Execute a single work item by spawning an implementation subagent with the work item brief and reference docs. Use when the user says "work on task N", "work on item N", "start the next task", "execute work item N", or invokes /swc-execute.
allowed-tools: Read, Agent, Bash
---

# SWC Execute

Delegate a single work item to a fresh implementation subagent. The main session never implements — it briefs, delegates, and receives.

## Arguments

- `/swc-execute` — pick up the next unchecked work item from the active workload
- `/swc-execute N` — execute a specific work item number

## Steps

### 1. Resolve the active workload

1. Run `git branch --show-current`
2. Read `.claude/.swc/meta.json`
3. Look up branch in `workloads` map → folder name
4. Fallback: most recently modified folder under `.claude/.swc/`

### 2. Resolve scope

- If work item number not supplied, find the first unchecked `- [ ]` item in `.claude/.swc/<folder>/workload.md`
- Confirm scope with the user before proceeding: "I'll work on **work item M**: [name]. Proceed?"

### 3. Load reference docs

Read in parallel:
- `.claude/.swc/<folder>/workload.md` — extract the full work item entry
- `.claude/.swc/<folder>/plan.md` — if it exists
- `.claude/.swc/<folder>/architecture.md` — if it exists

### 4. Compose the work item brief

Assemble a brief for the implementation subagent:

```
## Work item brief

**Work item:** [name]
[description]

**Context:** [from work item entry]
**Done when:** [from work item entry]

## Plan
[contents of plan.md, or "not provided"]

## Architecture
[contents of architecture.md, or "not provided"]
```

### 5. Spawn implementation subagent

```
Agent(
  subagent_type: "general-purpose",
  description: "Implement work item: [name]",
  prompt: "[work item brief from step 4]

  Follow the implementation-workflow skill. Return a rich summary artifact when complete."
)
```

### 6. Receive and present the summary artifact

When the subagent returns:
- Present the summary artifact to the user
- Do not assess or edit it — that is the review subagent's job
- Ask: "Ready to run the review?"

## Key Principles

- One work item at a time — never spawn multiple implementation subagents in parallel
- Always confirm scope before spawning — wrong item = wasted work
- Never implement anything directly — delegate only
- The summary artifact travels intact to the review step

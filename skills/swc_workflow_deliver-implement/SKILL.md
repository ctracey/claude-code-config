---
name: swc_workflow_deliver-implement
description: Spawn implementation agent for a work item — assemble brief and delegate to a placeholder agent, then evaluate and report exit criteria. Third phase of the delivery conversation. Use when spawning an implementation agent, or when invoked via /swc-workflow-deliver-implement.
allowed-tools: Bash, Read, Write, Edit, Glob, Agent
---

# Deliver — Implement Stage

Spawns a fresh implementation agent for the active work item, then evaluates and reports exit criteria. This is the third stage of the deliver workflow.

## Context

The work item number and name are available from the calling context (set by `swc_workflow_deliver` before the workflow started). If not available, read the active workload via `swc_lookup` and ask the user which item to implement.

## Steps

### 1. Confirm the work item

Display the work item being handed to the implementation agent:

> "Spawning implementation agent for **[N]: [name]**."

### 2. Spawn the implementation agent

Use the Agent tool to spawn a general-purpose agent. Pass **only** the work item number and name — no file paths, no doc contents. The implementation workflow is responsible for discovering its own context via naming conventions.

Agent prompt:

```
You are an implementation agent for work item [N]: [name].

This is a placeholder agent. Announce that you are a placeholder implementation agent for work item [N]: [name], confirm the work item you received, and complete without implementing anything.
```

Wait for the agent to return.

### 3. Evaluate exit criteria

After the agent completes, evaluate all four exit criteria. Check `.swc/<folder>/workitems/<N>/` for any outputs the agent produced.

| Criterion | How to evaluate |
|-----------|----------------|
| Agent completed | Did the agent return without error? |
| Agent documented its progress | Does `context.md` exist at `.swc/<folder>/workitems/<N>/context.md`? |
| Summary report exists | Does a summary artifact exist at `.swc/<folder>/workitems/<N>/summary.md`? |
| Work item ready for review | Does `context.md` contain a completed pass section and no unresolved blockers? |

### 4. Report results

Display all four criteria with pass/fail status:

> **Implement stage — exit criteria:**
> - [pass/fail] Agent completed
> - [pass/fail] Agent documented its progress
> - [pass/fail] Summary report of implementation progress exists
> - [pass/fail] Work item implementation is ready for review

If using the placeholder agent, all four will be unmet — surface this explicitly:

> "The placeholder agent produced no outputs. These criteria will be met once the implementation workflow (1.4.4) is built."

### 5. Return control

Return to the orchestrator with the criteria evaluation result. Do not attempt to self-advance if criteria are unmet — the orchestrator handles the gate decision.

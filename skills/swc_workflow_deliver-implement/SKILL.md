---
name: swc_workflow_deliver-implement
description: Spawn implementation agent for a work item — pass work item number only, then evaluate and report exit criteria. Fourth stage of the delivery workflow. Use when spawning an implementation agent, or when invoked via /swc-workflow-deliver-implement.
allowed-tools: Bash, Read, Glob, Agent
---

# Deliver — Implement Stage

Spawns a fresh implementation agent for the active work item, then evaluates and reports exit criteria.

## Context

The work item number is available from the calling context (set by `swc_workflow_deliver` before the workflow started). If not available, read the active workload via `swc_lookup` and ask the user which item to implement. The implementation agent is responsible for discovering its own folder and work item name via `swc_lookup`.

## Steps

### 1. Confirm the work item

Display the work item being handed to the implementation agent:

> "Spawning implementation agent for **[N]: [name]**."

### 2. Spawn the implementation agent

Use the Agent tool to spawn a general-purpose agent. Pass only the work item number — the agent uses `swc_lookup` to discover the workload folder and `swc_workload` to find the work item name.

```
Agent(
  subagent_type: "general-purpose",
  mode: "bypassPermissions",
  description: "Implement work item [N]",
  prompt: "You are an implementation agent for work item [N].

Use the swc_lookup skill to find the active workload folder, then read workload.md to confirm the work item name.

Follow the swc_workflow_implement skill to complete this work item.

CONSTRAINTS — you may NOT:
- Run git commit, git push, git tag, or any variant
- Run gh commands (pull requests, issues, releases)
- Push or publish anything to a remote

You MAY: read and write files, run build/test/lint commands, invoke skills, spawn subagents for review.
Delivery and git operations happen after user review — your job ends at a working, tested implementation."
)
```

Wait for the agent to return.

### 3. Evaluate exit criteria

After the agent completes, check `.swc/<folder>/workitems/<N>/` for outputs.

| Criterion | How to evaluate |
|-----------|----------------|
| Agent completed | Did the agent return without error? |
| Agent documented its progress | Does `context.md` exist at `.swc/<folder>/workitems/<N>/context.md`? |
| Summary report exists | Does `summary.md` exist at `.swc/<folder>/workitems/<N>/summary.md`? |
| Work item ready for review | Does `context.md` contain a completed pass section and no unresolved blockers? |

### 4. Report results

Display all four criteria with pass/fail status:

> **Implement stage — exit criteria:**
> - [pass/fail] Agent completed
> - [pass/fail] Agent documented its progress (`context.md`)
> - [pass/fail] Summary report exists (`summary.md`)
> - [pass/fail] Work item ready for review

### 5. Return control

Return to the orchestrator with the criteria evaluation result. Do not attempt to self-advance if criteria are unmet — the orchestrator handles the gate decision.

## Exit criteria

**Done when:**
- Agent returned without error
- `context.md` exists with at least one completed pass section — confirms orient ran
- `summary.md` exists — confirms the agent reached and completed the summarise stage (full workflow ran to completion)
- No unresolved blockers in `context.md`

**Return control to the calling skill.**

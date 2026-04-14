---
name: swc_workflow_implement-summarise
description: Summarise stage of the implementation workflow — complete context.md pass section, write summary artifact, return to deliver workflow. Fourth and final stage of the implementation workflow. Use when invoked by swc_workflow_implement or via /swc-workflow-implement-summarise.
allowed-tools: Read, Write, Glob, Bash
---

# Implement — Summarise Stage

**Partial implementation** — context.md pass enforcement is implemented. Summary artifact writing is deferred to 1.4.4.4.

## Steps

### 1. Announce stage entry

> "Summarise stage — Work item: [N]: [name]. Verifying context.md pass section."

### 2. Verify context.md pass section

Read `.swc/<folder>/workitems/<N>/context.md`. Find the current pass section (the last `## Pass` header).

If the current pass section has no bullet entries, surface this before exiting:

> "The context.md pass section for this run has no entries. Before wrapping up, capture what happened — even briefly: what was done, any decision or assumption made, or where things were left. This is the record a future agent relies on."

Do not return control to the orchestrator until at least one bullet entry exists under the current pass section.

### 3. Placeholder — remainder of summarise stage

The following logic is deferred to 1.4.4.4:
- Write summary artifact to `.swc/<folder>/workitems/<N>/summary.md`

Return control to the orchestrator.

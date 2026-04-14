---
name: swc_workflow_implement-orient
description: Orient stage of the implementation workflow — read the full brief, understand the starting point, open a new pass section in context.md. First stage of the implementation workflow. Use when invoked by swc_workflow_implement or via /swc-workflow-implement-orient.
allowed-tools: Read, Write, Glob, Bash
---

# Implement — Orient Stage

**Partial implementation** — context.md pass opening is implemented. Broader brief-reading logic is deferred to 1.4.4.4.

## Steps

### 1. Announce stage entry

> "Orient stage — Work item: [N]: [name]. Opening context.md pass section."

### 2. Open context.md pass section

Locate `.swc/<folder>/workitems/<N>/context.md`.

If it exists, read it to understand prior passes — what was tried, decided, and where things were left. Count existing `## Pass` headers to determine the next pass number.

If it does not exist, this is pass 1.

Append to context.md (create the file if needed):

```markdown
## Pass <N> — <YYYY-MM-DD>
```

Do not pre-fill entries — this section is filled during the implement stage at decision points.

### 3. Placeholder — remainder of orient stage

The following logic is deferred to 1.4.4.4:
- Read all brief docs (requirements, specs, solution, quality-baseline, plan, architecture)
- Understand the starting point from prior context and codebase state

Return control to the orchestrator.

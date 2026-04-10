---
name: swc_init
description: Scaffold a new .swc/<folder>/ with stub planning docs. Use when starting a fresh piece of work after the folder has been located by swc_lookup.
allowed-tools: Write
---

# SWC Init

Create the stub planning docs for a new workload folder. Called by `swc_workflow_plan-context` after `swc_lookup` has confirmed the folder path.

## Arguments

Receives the resolved folder path, e.g. `.swc/feature_my-work/`.

## Steps

### 1. Create stub files

Write the following files into `.swc/<folder>/`. Each file gets a title and section headers only — no content.

**`workload.md`**
```markdown
# Workload

## Work Items

```

**`plan.md`**
```markdown
# Plan

## Goal

## Background

## Approach

## Open Questions

```

**`architecture.md`**
```markdown
# Architecture

## Context

## Design

## Decisions

## Constraints

```

**`notes.md`**
```markdown
# Notes

## Decisions

## Risks

## References

```

**`changelog.md`**
```markdown
# Changelog

```

### 2. Return

Return the folder path to the calling skill. Print nothing — the calling skill handles confirmation.

## Exit criteria

**Done when** all five stub files exist at `.swc/<folder>/`.

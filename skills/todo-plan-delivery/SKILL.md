---
name: todo-plan-delivery
description: Understand the delivery shape — phases, milestones, and priorities. What lands first and why. Fourth phase of the planning conversation. Use when discussing rollout, sequencing, or priorities, or when invoked via /todo-plan-delivery.
allowed-tools: Read, Write, Edit, Glob
---

# Plan Delivery

Get a sense of the high-level shape before breaking into tasks. Brief — a couple of questions at most.

## Steps

### 1. Open

> "Before we break this into tasks — how do you see this unfolding? Are there phases, milestones, or a particular order you have in mind?"

### 2. Listen for the shape

Probe gently for the pattern the user has in mind:

| Shape | What it sounds like |
|---|---|
| Feature maturity | "Get the basics working, then add X, then polish" |
| Learning-driven | "Validate X before committing to Y" |
| Staged rollout | "MVP first, then layer in the full thing" |
| Dependency-ordered | "Can't do B until A is in place" |
| Priority-first | "The most important thing is X — everything else can wait" |

Don't categorise it — just understand what the user considers high priority and whether there are natural phases.

### 3. Play back

> "So the shape looks like: [summary]. The most important thing to land first is [X]. That right?"

### 4. Capture

Add `## Delivery shape` to `_plan.md` with 2–4 bullets summarising phases and priorities.

Write a skeleton task list to `todo-pr-N.md` — one top-level task per phase or priority area identified, no subtasks yet. This gives a starting shape for the detailed breakdown.

### 5. Present and check

Run `todo-list` to show the skeleton. Then ask:

> "Here's the rough shape as tasks. Does this ordering and grouping look right before we break it down further?"

Wait for confirmation or adjustments.

Transition: *"Good — let's figure out how to break it down."*

## Exit criteria

**Done when:**
- `_plan.md` has `## Delivery shape`
- Skeleton task list written to `todo-pr-N.md`
- User confirmed the ordering and grouping

**Return control to `todo-begin`.**

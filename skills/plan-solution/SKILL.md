---
name: plan-solution
description: Understand the solution direction for a piece of work — tech approach, constraints, open questions, and deferred decisions. Third phase of the planning conversation. Use when exploring how to build something, or when invoked via /plan-solution.
allowed-tools: Read, Write, Edit, Glob
---

# Plan Solution

Explore how to achieve the intent. This is a separate conversation from why — don't conflate goals with approach.

## Steps

### 1. Open

> "Now that we know what we're after — do you have a direction in mind for the solution, or would you like to explore options?"

### 2. Explore direction

Depending on the work, cover what's relevant — not all of these:

**Product / UX**
- What does the experience look like? Any sketches, references, or analogies?
- What's the simplest version that delivers the outcome?

**Technical**
- Architectural constraints or preferences?
- Existing patterns in the codebase to follow or avoid?
- Dependencies, integrations, or APIs involved?
- Performance, security, or scalability considerations?

### 3. Surface open questions and deferred decisions

Two distinct categories — name them differently in the docs:

- **Open question** — unresolved; we need to figure this out
- **Deferred decision** — intentionally parked; acknowledged and named, with enough context for a future conversation to pick it up

### 4. Play back

> "So the approach is [direction]. Key constraints are [X]. Still open: [questions]. Sound right?"

### 5. Capture

Write to `_architecture.md`:
- `## Tech stack`, `## Folder structure`, `## Constraints`

Write to `_notes.md`:
- `## Solution decisions` — key choices made and why
- `## Open questions` — unresolved items
- `## Deferred decisions` — intentionally parked, with context

Update `_plan.md`:
- `## Features` — what we're building
- `## Out of scope` — what we're not doing

Transition: *"Good — before we get into individual tasks, let's talk about how you see this unfolding."*

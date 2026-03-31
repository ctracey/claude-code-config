---
name: swc_workflow_plan-intent
description: Understand the intent behind a piece of work — why it exists, for whom, and what success looks like. Second phase of the planning conversation. Use when exploring motivation and goals, or when invoked via /swc-workflow-plan-intent.
allowed-tools: Read, Write, Edit, Glob
---

# Plan Intent

Understand why this work exists before discussing how to do it. Intent and solution are separate conversations.

## Steps

### 1. Open

> "Before we get into what we're building — what's driving this? What outcome or change are you trying to create?"

### 2. Explore

Calibrate depth to the complexity of the work. For small work items, one or two exchanges is enough. Draw from these selectively — not as a checklist:

- What problem are we solving, and for whom?
- What's the motivation — what's happening now that makes this needed?
- What does success look like? What would be different when this is done?
- Who are the users or personas affected? What are their goals or pain points?
- Are there specific scenarios or user journeys we need to support?
- Are there known constraints or things we must not break?

### 3. Check depth

> "How detailed do you want the plan — quick breakdown to get moving, or a thorough exploration of requirements first?"

Match the depth of the rest of the conversation to this answer.

### 4. Play back

> "So if I've got this right: [goal], for [who], because [why]. The key outcome is [what changes]. Does that capture it?"

Correct and re-confirm if needed.

### 5. Capture

Write to `plan.md`:
- `## Goal / Why` — one paragraph: what this accomplishes and the motivation

If users, personas, or scenarios were discussed, add `## Users and scenarios`.

If constraints were raised, add `## Constraints` to `notes.md`.

### 6. Present and check

Run `swc-report-plan` to show the current state of the plan doc. Then ask:

> "Does that capture what you're going for? Anything to adjust before we move to the solution?"

Wait for confirmation or corrections before transitioning.

Transition: *"Good — now let's talk about how you're thinking of approaching it."*

## Exit criteria

**Done when:**
- `plan.md` has `## Goal / Why` written
- User confirmed the playback is correct

**Return control to `swc-workflow-plan`.**

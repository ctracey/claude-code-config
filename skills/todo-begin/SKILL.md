---
name: todo-begin
description: Begin a new piece of work — scaffold todo + plan + architecture from a feature description. Use when starting a new project, plan, or piece of work, or when the user says "begin new work", "start a new plan", "new project", "new piece of work", or invokes /todo-begin.
allowed-tools: Bash, Read, Write, Edit, Glob
---

# Todo Begin

Entry point for starting a new piece of work. Runs the full planning conversation directly in the main session — no agent spawning.

## Arguments

- `/todo-begin` — auto-detect PR number from current branch
- `/todo-begin N` — use PR number N

## Steps

This skill owns the sequence. Invoke each phase skill in order via the `Skill` tool. When a phase reaches its exit criteria and returns control here, invoke the next one. Do not skip phases.

**Every phase must be invoked — no exceptions.** Compression applies to the depth of conversation within a phase (fewer questions, shorter exchanges), never to whether the phase runs. Exit criteria for each phase are non-negotiable: a phase is not done until its exit criteria are satisfied, regardless of how simple the work is.

Write to the docs as agreements are reached throughout — not at the end.

1. Invoke `todo-plan-context` — resolve PR number (pass the argument if one was given), check existing files, create stub docs
2. Invoke `todo-plan-intent` — why this work exists, for whom, what success looks like
3. Invoke `todo-plan-solution` — approach, constraints, open questions, deferred decisions
4. Invoke `todo-plan-delivery` — phases, milestones, priorities
5. Invoke `todo-plan-breakdown` — task list, confirm before writing
6. Invoke `todo-plan-finalise` — fill gaps, run todo-report playback, get user confirmation

When `todo-plan-finalise` returns control, planning is complete. Tell the user to run `/todo-execute` to begin the first task.

## Role boundary

**Plan. Do not implement.**

Implementation does not start until the user has explicitly confirmed the plan is correct — that confirmation is what `plan-finalise` is waiting for. The playback in `plan-finalise` is the gate: present it, wait for the user to say it looks right, then hand back. A plan the user hasn't confirmed is not a finished plan.

Once confirmed, return control. The user runs `/todo-execute` to begin the first task.

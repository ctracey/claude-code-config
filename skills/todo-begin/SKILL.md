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

## Stages

This skill owns the sequence. Invoke each stage skill in order via the `Skill` tool. When a stage reaches its exit criteria and returns control here, invoke the next one. Do not skip stages.

**Every stage must be invoked — no exceptions.** Compression applies to the depth of conversation within a stage (fewer questions, shorter exchanges), never to whether the stage runs. Exit criteria for each stage are non-negotiable: a stage is not done until its exit criteria are satisfied, regardless of how simple the work is.

Write to the docs as agreements are reached throughout — not at the end.

**Before invoking each stage skill**, emit a progress banner via the `todo-workflow-progress` skill with `title="planning"`, `stages="context,intent,solution,delivery,breakdown,finalise"`, and `active=<stage name>` as listed below. When all stages are done, emit one final banner with `active=""`.

1. `context` — Invoke `todo-plan-context` — resolve PR number (pass the argument if one was given), check existing files, create stub docs
2. `intent` — Invoke `todo-plan-intent` — why this work exists, for whom, what success looks like
3. `solution` — Invoke `todo-plan-solution` — approach, constraints, open questions, deferred decisions
4. `delivery` — Invoke `todo-plan-delivery` — phases, milestones, priorities
5. `breakdown` — Invoke `todo-plan-breakdown` — task list, confirm before writing
6. `finalise` — Invoke `todo-plan-finalise` — fill gaps, run todo-report playback, get user confirmation

When `todo-plan-finalise` returns control, emit the final banner (`active=""`, all stages done), then tell the user to run `/todo-execute` to begin the first task.

## Role boundary

**Plan. Do not implement.**

Implementation does not start until the user has explicitly confirmed the plan is correct — that confirmation is what `plan-finalise` is waiting for. The playback in `plan-finalise` is the gate: present it, wait for the user to say it looks right, then hand back. A plan the user hasn't confirmed is not a finished plan.

Once confirmed, return control. The user runs `/todo-execute` to begin the first task.

---
name: todo-begin
description: Begin a new piece of work — scaffold todo + plan + architecture from a feature description. Use when starting a new project, plan, or piece of work, or when the user says "begin new work", "start a new plan", "new project", "new piece of work", or invokes /todo-begin.
allowed-tools: Bash
---

# Todo Begin

Entry point for starting a new piece of work. Spawns the `todo-plan` agent to run the planning conversation.

## Arguments

- `/todo-begin` — auto-detect PR number from current branch
- `/todo-begin N` — use PR number N

## Steps

### 1. Resolve PR number

If a number was passed as an argument, that is the PR number. Otherwise pass `auto` — the `todo-plan` agent will detect it.

### 2. Spawn the planning agent

Spawn the `todo-plan` agent with:
- The PR number (or `auto`)
- The current working directory

The agent runs the full planning conversation — context check, intent, solution direction, delivery shape, task breakdown, and final review. It writes directly to the planning docs throughout.

Do not run any planning steps in this skill. Hand off immediately.

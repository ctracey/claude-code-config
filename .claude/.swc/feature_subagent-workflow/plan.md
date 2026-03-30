# Claude Code task execution workflow

## Overview

This document describes a structured approach to delegating software development tasks through Claude Code using a main session, an implementation subagent, and a review subagent. The workflow is sequential — one task at a time — with the user present to approve outcomes and resolve decision points.

The design prioritises clean context, quality gates, and user control at every stage.

---
## Actors and responsibilities

These are the main actors that support this approach:

 - Main session
 - Implementation subagent
 - Review subagent

### Main session

The main session is the orchestrator. It holds the project context and drives the task lifecycle from start to finish.

**Responsibilities:**
- Maintain the three reference documents (`todo.md`, `plan.md`, `architecture.md`)
- Trigger the task execution skill to begin each task
- Receive and assess the rich summary artifact from the implementation subagent
- Spawn the review subagent and provide it with the summary artifact
- Assess review findings and, where needed, spawn a fresh implementation subagent to apply fixes
- Present the structured user handoff and await approval
- Commit and push to the GitHub PR branch on task completion
- Move to the next task once the user approves

The main session never implements code. It delegates, coordinates, and decides.



### Implementation subagent

The implementation subagent is spawned by the main session for each task or fix cycle. It operates within its own session context, meaning the WAY fires fresh, preferences are clean, and there is no accumulated context baggage from prior tasks.

**Responsibilities:**
- Receive the task brief from the main session (task description from `todo.md`, plus `plan.md` and `architecture.md` for context)
- Follow the implementation skill workflow
- Surface decision points to the user rather than making assumptions
- Verify tests pass and the build is clean before completing
- Produce a rich summary artifact covering: what was done, decisions made, approach taken, tests added, build status
- **Proactively make the case for quality** — don't wait for the reviewer to ask. Lead with why the solution is sound: approach rationale, edge cases considered, test coverage, known trade-offs. The reviewer's job is verification, not extraction.
- Return the summary artifact to the main session

The implementation subagent does not directly invoke the review subagent. That responsibility sits with the main session.



### Review subagent

The review subagent is spawned by the main session after the implementation subagent returns its summary. It operates in its own fresh session, so the WAY fires independently with clean preferences.

**Responsibilities:**
- Receive the rich summary artifact and relevant code context from the main session
- Conduct a code review against the project's standards and preferences (injected via WAY)
- Return structured findings to the main session

The review subagent does not apply fixes. It reports findings and returns control to the main session.

---
## Workflow sequence

```
Main session
  └── Trigger task execution skill
        └── Spawn implementation subagent
              ├── WAY fires: inject project preferences
              ├── Clarify requirements (surface decisions to user if needed)
              ├── Implement
              ├── Test + build check
              └── Return rich summary artifact to main

Main session
  └── Spawn review subagent (with summary artifact)
        ├── WAY fires: inject project preferences
        ├── Code review
        └── Return findings to main

Main session
  ├── Fixes needed?
  │     └── Spawn fresh implementation subagent with findings
  │           └── Apply fixes → return updated summary
  │
  └── Structured user handoff
        ├── What was done
        ├── Decisions made and approach taken
        ├── Tests added + all tests passing
        ├── Build status
        ├── Code review completed, findings addressed
        └── Artifact presented for user approval

User approves → commit + push to PR → next task
```

---
## Technical solutions

This solution onboards the agent by:

 - using claude code extensible mechanisms that influence agent behaviour
 - customising workflows & behaviour

### WAY (context injection mechanism)

A WAY is a session-scoped context injection pattern defined in `CLAUDE.md`. A hook at session start watches for a pattern match in the user's prompt. The first time a matching pattern is detected, the relevant context is injected — once per session only.

Because each subagent has its own session, the WAY fires fresh for both the implementation subagent and the review subagent. This ensures preferences are always present and not lost due to context drift in a long main session.

**Example WAY content:**
- Source code lives in `src/`, test files in `tests/`
- Tech stack constraints and folder conventions
- Code style preferences


### Skills

Skills define reusable, repeatable processes that can be triggered explicitly (via slash command) or implicitly when Claude Code infers the intent.

| Skill | Purpose |
|---|---|
| Task execution skill | Triggered from main session; spawns the implementation subagent and passes reference docs |
| Implementation workflow skill | Governs the step-by-step process: clarify → implement → test → build check → summarise |
| User handoff skill | Defines the structured handoff format: summary first, then decisions, tests, review status, artifact |



### Hooks

| Hook | Trigger | Purpose |
|---|---|---|
| Session start hook | Start of every session | Initialises WAY pattern matching |
| Pre-commit / pre-push hook | Before git operations | Confirms user is ready before code is committed or pushed |



### Subagent definitions

| Subagent | Spawned by | Scope |
|---|---|---|
| Implementation subagent | Main session (via task execution skill) | Implements a single task following the implementation workflow skill |
| Review subagent | Main session (after implementation returns) | Reviews code and summary artifact; returns findings only |

Each subagent gets its own session, its own WAY injection, and operates independently. Fresh subagents are always preferred over reuse.


---
## Reference documents

Reference docs live in the `.swc/` workload folder for the branch. All scoped to the branch/PR.

| File | Contents |
|---|---|
| `.claude/.swc/<folder>/workload.md` | Numbered task breakdown with status markers |
| `.claude/.swc/<folder>/plan.md` | Feature list, goals, product intent, out of scope |
| `.claude/.swc/<folder>/architecture.md` | Tech stack decisions, folder structure, architectural constraints |
| `.claude/.swc/<folder>/notes.md` | Conventions, agreements, decisions that apply across tasks |
| `.claude/.swc/<folder>/changelog.md` | Append-only per-task record of what happened and why |

### `workload.md` task format

```markdown
- [ ] **N. Task name**
  - [ ] N.1. Sub-task description
```

### `plan.md` format

```markdown
## Goal / Why
## Features
## Out of scope
```

### `architecture.md` format

```markdown
## Tech stack
## Folder structure
## Constraints
```

---

### GitHub integration

- Each task completion triggers a commit and push to the PR branch
- Tags or commit markers track task boundaries
- Commit history provides a rollback point if a task is rejected
- The pull request represents the holistic outcome of the main session


---
## Decision points and user control

The implementation skill explicitly requires decision points to be surfaced to the user rather than resolved autonomously. The subagent presents options and a recommendation — the user makes the final call.

---
## Key design principles

**Sequential over parallel** — one task at a time, user approves before the next begins.

**Small tasks as discipline** — smaller tasks are clearer, less abstract, and more specific. Tight feedback loops catch drift early and prevent waste on work that isn't needed.

**Main session as orchestrator only** — never implements, only delegates and coordinates.

**Fresh subagents over session reuse** — clean context, fresh WAY injection, no accumulated drift.

**Rich summary as the handoff artifact** — context travels explicitly between actors, not via shared session state.

**Proactive quality pitch** — the implementation subagent arrives at handoff having already made the case for quality. The reviewer verifies; it doesn't interrogate.

**Structured user handoff** — always leads with what was done, not a question.

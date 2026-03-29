# PR-5.1 Notes — Planning agent architecture

## Scope

This sub-initiative refactors `todo-begin` from a monolithic 8-step skill into a layered architecture: a thin entry point that spawns a dedicated planning agent, which in turn uses focused per-phase skills.

Parent: PR-5 task 3.1 (`todo-begin` skill)

---

## Three core agreements

### 1. The docs are the only briefing the implementer gets

The planning conversation happens in an isolated agent session. The implementation subagent — and any future session — will not have access to that conversation. The documents are the only thing they will see.

Every influential decision, constraint, and intent must be captured, along with why. Not a transcript — complete enough that someone who was not in the room can read the docs and do the work correctly. If it shaped the direction, it belongs in the docs. If it was just conversational, leave it out. When in doubt, include it.

**Gap vs deferred decision:** The distinction matters. A gap is something missed. A deferred decision is something acknowledged and intentionally parked — "we'll work out the user story format when we get to that task". Deferred decisions must be named explicitly, with enough context to pick them up later. Both are first-class entries. The difference must be clear in the docs.

### 2. Write to the docs throughout, not at the end

As each agreement is reached in the planning conversation, it gets written to the relevant doc immediately. The final review step is just confirming nothing was missed — not a documentation event. If the session ends unexpectedly, the docs should reflect everything agreed up to that point.

### 3. Collaboration principles belong in a way

The planning conversation is guided by principles that apply across all planning phases. These belong in a way (`hooks/ways/meta/planning/way.md`) that fires automatically in the agent's session — not embedded in individual skill files. This keeps skills lean and ensures principles apply uniformly regardless of which phase skill is active.

---

## Collaboration principles (full)

These move to the planning way. Captured here as the canonical reference.

**Ask, don't assume.** Lead with questions. Don't fill in blanks the user hasn't given you.

**One question at a time.** Don't front-load a list of questions. Ask the most important one, listen, then follow up if needed.

**Play back before moving on.** At the end of each step, briefly reflect the picture back. A single sentence: *"So the goal is X, and the main thing you're solving for is Y — does that sound right?"* Correct and re-confirm before proceeding.

**Read the room on depth.** Short answers → stay high-level and move forward. Elaborating → follow deeper. Match their energy and vocabulary. Explicitly ask at intent step how much detail they want.

**Show the picture building.** At natural transition points (intent→solution, solution→delivery, delivery→breakdown), briefly surface what you've understood so far — a one or two sentence summary — before asking the next question. Helps the user see the plan forming and correct early.

**Be mindful of their time.** If the work is simple and the user clearly knows what they want, compress. Multiple steps can collapse into a single short exchange. The process should scale down for small work, not be a ritual that must be completed in full.

---

## Architecture

### Layer 1 — Entry point: `todo-begin` skill

Thin orchestrator. Resolves the PR number from the argument (if provided) and spawns the `todo-plan` agent immediately with context. Does not run any planning steps itself.

Invocation:
- `/todo-begin` — auto-detect PR number from branch/PR
- `/todo-begin N` — use specific PR number N

### Layer 2 — Planning agent: `todo-plan`

First-class agent definition at `agents/todo-plan.md`. Runs the full planning conversation by invoking the plan-* skills in sequence. Receives PR number and working directory as context.

Rationale for first-class agent over inline skill: semantically discoverable, independently improvable, consistent with the pattern of `code-reviewer`, `task-planner`, etc.

Ways fire fresh for every subagent session. The planning way — and any other relevant ways — will be active in `todo-plan`'s session from the start, giving it clean context without relying on the main session's history.

### Layer 3 — Phase skills

Six focused skills, each owning one phase of the planning conversation. Each is independently invocable — useful for resuming a planning session at a specific phase.

| Skill | Phase | Writes to |
|---|---|---|
| `plan-context` | Branch/PR check, existing todos, stub doc creation, relate to existing work | stub docs |
| `plan-intent` | Why, for whom, what success looks like | `_plan.md` Goal/Why, Users/scenarios |
| `plan-solution` | Tech direction, constraints, open questions, deferred decisions | `_architecture.md`, `_notes.md` |
| `plan-delivery` | Phases, milestones, priorities | `_plan.md` Delivery shape |
| `plan-breakdown` | Navigation style, task list proposal, confirmation, write | `todo-pr-N.md` |
| `plan-finalise` | Fill gaps, run `todo-report`, confirm ready | all docs |

### Layer 4 — Planning way

`hooks/ways/meta/planning/way.md` — fires in the agent's session and injects the collaboration principles. Keeps skill files lean.

---

## File layout

```
agents/
  todo-plan.md                  ← planning agent definition

skills/
  todo-begin/SKILL.md           ← entry point (updated: thin orchestrator)
  plan-context/SKILL.md         ← phase 1: context + existing-work mode
  plan-intent/SKILL.md          ← phase 2: intent and motivation
  plan-solution/SKILL.md        ← phase 3: solution direction
  plan-delivery/SKILL.md        ← phase 4: delivery shape and priorities
  plan-breakdown/SKILL.md       ← phase 5: task breakdown
  plan-finalise/SKILL.md        ← phase 6: fill gaps, review, confirm

hooks/ways/meta/planning/way.md ← collaboration principles way
```

---

## Phase detail

### plan-context

Resolves the PR number (argument takes precedence over detection). Checks current branch and open PRs.

| Branch state | Action |
|---|---|
| On `main` | Note — a new branch will be needed before work begins |
| Feature branch, no PR | Ask if this initiative relates to that branch or needs a fresh one |
| Feature branch with open PR | Confirm: "Is this new work for PR #N, or a separate initiative?" |

If too early to create a branch/PR, use a timestamp placeholder: `todo-pr-YYYYMMDD.md` — rename when a real PR exists.

Checks `.claude/` for existing `todo-pr-*.md` files. If found, surfaces a brief summary (PR number, task count, done count) and asks how the new work relates:

1. **Replace** — archive (rename with `_archived` suffix) or discard existing docs, start fresh
2. **Extend** — add new tasks continuing from the highest existing task number; ask how the new work relates (goes into `_notes`)
3. **Sibling** — renumber existing tasks one level deeper, new work becomes a peer task at top level; ask for a label for the existing work group
4. **New sections** — keep task list as-is, add named sections to existing `_notes` and/or `_architecture`; skip to plan-delivery

**Sibling renumbering logic:**
- Old top-level task `1` with subtasks `1.1`, `1.2` → becomes `1.1` with subtasks `1.1.1`, `1.1.2`
- Old top-level task `2` with subtask `2.1` → becomes `1.2` with subtask `1.2.1`
- New work becomes task `2` with its own breakdown

Creates stub docs early (title + section headers only) so subsequent phases can append incrementally:
- `todo-pr-N.md`
- `todo-pr-N_plan.md`
- `todo-pr-N_architecture.md`
- `todo-pr-N_notes.md`

For Extend/Sibling/New sections modes, edit existing files rather than creating new ones.

### plan-intent

Establishes why before what. Keeps intent and solution separate.

Opens with: *"Before we get into what we're building — what's driving this? What outcome or change are you trying to create?"*

Intent questions to draw from selectively (not as a checklist):
- What problem are we solving, and for whom?
- What's the motivation — what's happening now that makes this needed?
- What does success look like? What would be different when done?
- Who are the users or personas affected?
- Are there specific scenarios or user journeys to support?
- Are there known constraints or things we must not break?

Checks how deep to go: *"How detailed do you want the plan — quick breakdown to get moving, or a thorough exploration first?"*

Playback: *"So if I've got this right: [goal], for [who], because [why]. The key outcome is [what changes]. Does that capture it?"*

Captures to `_plan.md`: Goal/Why section. Users/scenarios section if discussed. Constraints to `_notes.md` if raised.

Transition: *"Good — now let's talk about how you're thinking of approaching it."*

### plan-solution

Shifts to how. Keeps solution separate from intent — don't conflate what with how.

Opens with: *"Now that we know what we're after — do you have a direction in mind for the solution, or would you like to explore options?"*

Explores as relevant — product/UX direction (experience, flows, simplest version) or technical direction (architecture, existing patterns, dependencies, constraints). Doesn't ask all of these — picks what the work calls for.

Surfaces open questions (unresolved, need to figure out) vs deferred decisions (intentionally parked with context for future pickup) as distinct entries.

Playback: *"So the approach is [direction]. Key constraints are [X]. Still open: [questions]. Sound right?"*

Captures to:
- `_architecture.md` — tech stack, folder structure, constraints
- `_notes.md` — `## Solution decisions`, `## Open questions`, `## Deferred decisions`
- `_plan.md` — Features and Out of scope sections

Transition: *"Good — before we get into individual tasks, let's talk about how you see this unfolding."*

### plan-delivery

Brief — gets a sense of shape before task granularity. Not a planning session in itself.

Opens with: *"Before we break this into tasks — how do you see this unfolding? Are there phases, milestones, or a particular order you have in mind?"*

Patterns to probe gently: feature maturity, learning-driven, staged rollout, dependency-ordered, priority-first.

Playback: *"So the shape looks like: [summary]. The most important thing to land first is [X]. That right?"*

Captures to `_plan.md`: `## Delivery shape` section, 2–4 bullets.

Transition: *"Good — let's figure out how to break it down."*

### plan-breakdown

Asks how the user wants to navigate task organisation:

| Approach | What it means |
|---|---|
| Logical / technical map | Tasks follow system structure — components, layers, modules |
| User / persona | Tasks grouped by who benefits |
| Feature set | Tasks grouped by capability area |
| Journey / scenario | Tasks follow an end-to-end flow |

User may mix these. Follow their lead — don't impose structure.

Proposes the full task list as plain text using the `todo-list` visual format. For Sibling mode, shows the full renumbered list before any files change.

Asks: *"Does this numbering and breakdown look right? Say yes to write the files, or tell me what to adjust."*

Waits for explicit confirmation. Iterates if requested. Does not write until confirmed.

Captures confirmed list to `todo-pr-N.md`. Task granularity guidance: small and specific, clear "done when" implied by name, more than ~5 subtasks under one parent → consider splitting.

### plan-finalise

Reviews all four docs for completeness. Fills remaining gaps. For Extend/Sibling/New sections modes, edits rather than overwrites.

Checklist:
- `todo-pr-N.md` — task list present and confirmed?
- `_plan.md` — Goal/Why, Features, Out of scope, Delivery shape?
- `_architecture.md` — Tech stack, Folder structure, Constraints?
- `_notes.md` — Doc purpose table, Solution decisions, Open questions, Deferred decisions?

Runs `todo-report`. Invites user to check docs directly if anything looks off. Confirms ready to start.

---

## Doc templates

### `todo-pr-N.md`
```markdown
# PR-N Task Breakdown — [work title]

## Tasks

- [ ] **1. [First task]**
  - [ ] 1.1. [Subtask]
  - [ ] 1.2. [Subtask]

- [ ] **2. [Second task]**
  - [ ] 2.1. [Subtask]
```

### `todo-pr-N_plan.md`
```markdown
# PR-N Plan — [work title]

## Goal / Why
[One paragraph — what this accomplishes and the motivation behind it]

## Features
- [Feature one]
- [Feature two]

## Out of scope
- [What we're not doing]

## Delivery shape
- [Phase or priority bullets]
```

### `todo-pr-N_architecture.md`
```markdown
# PR-N Architecture — [work title]

## Tech stack
[Languages, frameworks, key libraries — or "not yet determined"]

## Folder structure
[Relevant directories and what lives where — stub if unknown]

## Constraints
[Hard limits: must use X, must not touch Y, performance requirements, etc.]
```

### `todo-pr-N_notes.md`
```markdown
# PR-N Notes — [work title]

## Doc purpose

| Doc | Purpose |
|-----|---------|
| `_plan` | What and why — goals, features, scope, intent. Written upfront. |
| `_notes` | How — conventions, agreements, decisions that apply across tasks. Stable reference. |
| `_changelog` | What happened — append-only per-task record of decisions, changes, and learnings. |
| `_architecture` | Tech stack, folder structure, hard constraints. |

## Solution decisions
[Key choices made and why]

## Open questions
[Unresolved items — things we need to figure out]

## Deferred decisions
[Intentionally parked items — acknowledged and named, with context for future pickup]
```

---

## Key principles

- Never overwrite existing content without explicit confirmation
- Small tasks over large ones — clearer, tighter feedback loops
- The `_plan` doc is written upfront and should not change during execution; new scope goes in `_notes`
- When in doubt about task granularity, err smaller — tasks can always be grouped, but coarse tasks are hard to delegate cleanly
- `_changelog.md` is created by the execution workflow (`todo-execute`), not the planning workflow

---

## Acceptance criteria

The planning conversation is complete when the user can close all sessions, return the next day, open a new session, and — using only the docs — pick up delivery with confidence. No re-explaining context, no re-describing the goal, no re-making decisions already taken.

A new session should be able to read the docs and immediately know:
- What we're building and why
- Who it's for and what they need
- What's in scope and what's explicitly out
- The approach agreed for the solution
- The delivery shape and priorities
- The task breakdown and what to start with
- What's been decided, what's an open question, and what's intentionally deferred

If any of these would require asking the user again, the docs are not complete.

Implementation acceptance:
- `todo-begin` invokes `todo-plan` agent immediately on launch
- `todo-plan` agent runs phases 1–6 in order using dedicated skills
- Docs are written incrementally throughout — not batched at the end
- Collaboration principles are in the way, not duplicated across skill files
- Each `plan-*` skill is independently invocable
- After a completed planning session, a new session can read the docs and start delivery without re-asking the user anything

---

## Open questions

- How are ways triggered within an agent session? Confirm that `scope: agent` in the planning way frontmatter causes it to fire when `todo-plan` starts.
- Should `plan-context` handle all four existing-work modes (replace/extend/sibling/new sections) inline, or delegate mode-specific logic to the agent?

---

## Deferred decisions

- Story mapping integration (task 9.1) — how `plan-delivery` and `plan-breakdown` could support outcome/learning milestones as an alternative to pure task lists. Deferred until the core architecture is stable.
- Changelog creation — confirmed: `_changelog.md` is created as part of the execution workflow (`todo-execute`), not planning. No action needed here.

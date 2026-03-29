---
name: todo-begin
description: Begin a new piece of work — scaffold todo + plan + architecture from a feature description. Use when starting a new project, plan, or piece of work, or when the user says "begin new work", "start a new plan", "new project", "new piece of work", or invokes /todo-begin.
allowed-tools: Read, Write, Edit, Glob
---

# Todo Begin

Scaffold a new piece of work. Creates (or extends) the three reference docs — `todo-pr-N.md`, `_plan`, `_architecture`, `_notes` — from a feature description.

## Arguments

- `/todo-begin` — interactive scaffold, auto-detect PR number
- `/todo-begin N` — use a specific PR number

---

## Acceptance criteria

The planning conversation is complete when the user can close all sessions, return the next day, open a new session, and — using only the docs — pick up delivery with confidence. No need to re-explain the context, re-describe the goal, or re-make decisions already taken.

A new session should be able to read the docs and immediately know:
- What we're building and why
- Who it's for and what they need
- What's in scope and what's explicitly out
- The approach agreed for the solution
- The delivery shape and priorities
- The task breakdown and what to start with
- What's been decided, what's an open question, and what's intentionally deferred

If any of these would require asking the user again, the docs are not complete.

---

## Core principle: the docs are the only briefing the implementer gets

The planning conversation happens in its own session. The implementation subagent — and any future session — will not have access to that conversation. The documents are the only thing they will see.

This means the docs must be self-contained and complete. Every decision that influences the work must be captured, along with why it was made. Every constraint, every intent, every direction agreed during planning. Not a transcript — but comprehensive enough that someone who was not in the room can read the docs and do the work correctly.

If it shaped the direction of the work, it belongs in the docs. If it was just conversational, leave it out. When in doubt, include it.

**Deferred decisions are first-class entries.** Not everything needs to be resolved in the planning conversation. If something is intentionally parked — "we'll work out the user story format when we get to that task", "the auth approach needs its own conversation" — capture it explicitly as a deferred decision with enough context to pick it up later. The distinction between a gap (something missed) and a deferred decision (something acknowledged and parked) must be clear in the docs.

Write to them as agreements are reached throughout the conversation. The final review is just confirming nothing was missed and every open item is explicitly named.

---

## Collaboration principles

This is a conversation, not a form. The goal is to think through the work together and arrive at a shared understanding.

**Ask, don't assume.** Lead with questions. Don't fill in blanks the user hasn't given you.

**One question at a time.** Don't front-load a list of questions. Ask the most important one, listen, then follow up if needed.

**Play back before moving on.** At the end of each step, briefly reflect the picture back — what you've understood so far. Ask if it's right before proceeding. A single sentence is enough: *"So the goal is X, and the main thing you're solving for is Y — does that sound right?"*

**Read the room on depth.** If the user is giving short answers, stay high-level and move forward. If they're elaborating, follow them deeper. Match their energy and vocabulary. Explicitly ask at step 3 how much detail they want to go into.

**Show the picture building.** At natural transition points (step 3→4, step 4→5, step 5→6), briefly surface what you've understood so far — a one or two sentence summary of intent, shape, and approach — before asking the next question. This helps the user see the plan forming and correct it early if something's off.

**Be mindful of their time.** If the work is simple and the user clearly knows what they want, compress the conversation. Steps 3–5 can be a single short exchange. The process should scale down for small work, not be a ritual that must be completed in full.

---

## Steps

### 1. Check context

**PR number from argument:** If a number was passed as an argument (`/todo-begin N`), that is the PR number — skip the branch/PR detection below and go straight to checking for existing todo files.

**Check the current branch:**

Run `git branch --show-current` and `gh pr view --json number,title 2>/dev/null`.

| State | Action |
|---|---|
| On `main` | Note this — a new branch will be needed before work begins |
| On a feature branch, no PR | Ask if this initiative relates to that branch or if they want a fresh one |
| On a feature branch with open PR | Confirm: "Is this new work related to PR #N (`[title]`), or a separate initiative?" |

If the user is ready to create a branch and PR now, do it as part of setup. If it's too early, use a timestamp as the PR number placeholder (e.g. `todo-pr-20260329.md`) — this can be renamed once a real PR exists.

**Check for existing todo files:**

Look for `todo-pr-*.md` files in `.claude/`. Read the most recently modified one if found.

**If no existing files:** skip to step 3.

**If existing files found:** surface a brief summary — PR number, how many tasks, how many done — then go to step 2.

> **Capture:** Create stub files for `todo-pr-N_plan.md`, `todo-pr-N_notes.md`, and `todo-pr-N_architecture.md` now (title + section headers only). This establishes the files early so subsequent steps can append to them incrementally.

---

### 2. Ask how this work relates

Present the options clearly:

> I found an existing todo list (`todo-pr-N.md`, M tasks, X done). How does this new work relate?
>
> 1. **Replace** — archive or discard the existing list and start fresh
> 2. **Extend** — add new tasks to the existing list under the same scope
> 3. **Sibling** — nest the existing tasks one level deeper and add this as a peer
> 4. **New sections** — keep the task list as-is but add sections to the existing `_notes` and `_architecture` docs

Wait for the user's choice before proceeding.

#### Option handling

**Replace**
- Ask: "What should happen to the existing docs? Archive them (rename with `_archived` suffix), or discard?"
- After confirming, proceed to step 3 as if starting fresh.

**Extend**
- Ask how the new work relates to the existing scope (one sentence — this goes into `_notes`).
- Proceed to step 3. New tasks will be appended continuing from the highest existing task number.

**Sibling**
- Renumber existing tasks: each current top-level task (1, 2, 3…) becomes a subtask of a new task 1.
  - Old task `1` with subtasks `1.1`, `1.2` → becomes `1.1` with subtasks `1.1.1`, `1.1.2`
  - Old task `2` with subtask `2.1` → becomes `1.2` with subtask `1.2.1`
  - Old top-level task name becomes a descriptive label for the new parent (e.g. `1. [existing work label]`)
- The new work becomes task `2` with its own breakdown.
- Ask: "What should the label be for the existing work as a group?" (e.g. "Backend refactor", "Phase 1")
- Proceed to step 3 to define the new work (task 2 and below).

**New sections**
- Ask what the new section is about (one sentence).
- Add a named section to `_notes` and/or `_architecture` as appropriate.
- Add any new tasks to the existing list (continuing from highest task number).
- Skip to step 4 (no new plan doc needed unless the user wants one).

---

### 3. Understand the intent

Before discussing solutions, establish why this work exists. Lead with a conversation, not a form.

Start with:
> "Before we get into what we're building — what's driving this? What outcome or change are you trying to create?"

Then explore as needed, calibrating depth to the complexity of the work. For a small task, one or two exchanges is enough. For larger or more ambiguous work, go deeper.

**Intent questions to draw from (use selectively, not as a checklist):**
- What problem are we solving, and for whom?
- What's the motivation or driver — what's happening now that makes this needed?
- What does success look like? What would be different when this is done?
- Who are the users or personas affected? What are their goals or pain points?
- Are there specific scenarios or user journeys we need to support?
- Are there known constraints, dependencies, or things we must not break?

Once you have enough to articulate the intent clearly, play it back in a sentence or two and ask if it's right. This becomes the **Goal / Why** in `_plan.md`.

**Check how deep to go:**
> "How detailed do you want the plan — quick breakdown to get moving, or a thorough exploration of requirements first?"

If they want to keep it light, capture intent in a few sentences and move on. If they want to go deeper, continue the conversation before moving to solution.

> **Capture:** Write the confirmed intent to `_plan.md` (Goal/Why section). If users, personas, or scenarios were discussed, add them under a `## Users and scenarios` section. Add any known constraints or must-nots to `_notes.md` as an `## Constraints` section.

**Playback before moving on:**
> "So if I've got this right: [one sentence on the goal], for [who], because [why]. The key outcome is [what changes]. Does that capture it?"

Correct and re-confirm if needed. Then transition:
> "Good — now let's talk about how you're thinking of approaching it."

---

### 4. Understand solution direction

Once intent is clear, shift to how. This is a separate conversation — don't conflate what we're trying to achieve with how we'll achieve it.

Open with:
> "Now that we know what we're after — do you have a direction in mind for the solution, or would you like to explore options?"

Depending on the work, this may be:

**Product / UX direction**
- What does the experience look like? Any sketches, references, or analogies?
- Are there flows or screens we know we need?
- What's the simplest version that delivers the outcome?

**Technical direction**
- Any architectural constraints or preferences?
- Existing patterns in the codebase to follow or avoid?
- Dependencies, integrations, or APIs involved?
- Any performance, security, or scalability considerations?

You don't need answers to all of these — just enough to inform the task breakdown. Capture key decisions and constraints; anything unresolved goes into `_notes` as an open question.

If the breakdown feels complex (many moving parts, unclear dependencies), offer to spawn a `task-planner` subagent to help reason about task order and structure. Present this as optional.

> **Capture:** Write tech stack, folder structure, and constraints to `_architecture.md`. Add key decisions to `_notes.md` under `## Solution decisions`. Add unresolved unknowns to `_notes.md` under `## Open questions`. Add intentionally parked items to `_notes.md` under `## Deferred decisions` — include enough context so a future conversation can pick them up. Update `_plan.md` Features and Out of scope sections.

**Playback before moving on:**
> "So the approach is [one sentence on the solution direction]. Key constraints are [X]. Still open: [any unresolved questions]. Sound right?"

Then transition:
> "Good — before we get into individual tasks, let's talk about how you see this unfolding."

---

### 5. Understand delivery shape

Before jumping into task granularity, get a sense of the high-level roadmap and what matters most. This is a brief conversation — not a planning session.

Open with:
> "Before we break this into tasks — how do you see this unfolding? Are there phases, milestones, or a particular order you have in mind?"

Listen for the shape the user has in mind. Common patterns to probe gently:

| Shape | What it sounds like |
|---|---|
| **Feature maturity** | "First get the basics working, then add X, then polish" |
| **Learning-driven** | "We need to validate X before committing to Y" |
| **Staged rollout** | "MVP first, then layer in the full thing" |
| **Dependency-ordered** | "We can't do B until A is in place" |
| **Priority-first** | "The most important thing is X — everything else can wait" |

You don't need to categorise it — just understand what the user considers high priority and whether there are natural phases or release points. A couple of questions is enough.

> **Capture:** Add a `## Delivery shape` section to `_plan.md` with 2–4 bullet points summarising the phases or priorities discussed. This informs the task ordering in the next step.

**Playback before moving on:**
> "So the shape looks like: [brief summary of phases/priorities]. The most important thing to land first is [X]. That right?"

Then transition:
> "Good — let's figure out how to break it down."

---

### 6. Propose task breakdown

Before drafting, ask how the user wants to navigate the breakdown:

> "How do you want to organise the tasks — do you have a logical map in mind, or would you prefer to walk through it by user, feature set, or journey/scenario?"

| Approach | What it means |
|---|---|
| **Logical / technical map** | Tasks follow the system structure — components, layers, modules |
| **User / persona** | Tasks grouped by who benefits or what they need to do |
| **Feature set** | Tasks grouped by capability area — each feature as a cluster |
| **Journey / scenario** | Tasks follow an end-to-end flow — what happens step by step |

The user may mix these (e.g. phases from step 5 as top-level, features within each phase). Follow their lead — don't impose a structure.

With the delivery shape (step 5) and chosen navigation in mind, draft the full task list. Present it for confirmation before writing.

Show the proposed numbering as plain text, using the same visual format as `todo-list`:

```
□ 1. First task
  □ 1.1. Subtask
  □ 1.2. Subtask
□ 2. Second task
  □ 2.1. Subtask
```

For **Sibling** mode, show the full renumbered list — existing tasks nested under `1.` and new work under `2.` — so the user can see the complete picture before any files change.

Ask: "Does this numbering and breakdown look right? Say yes to write the files, or tell me what to adjust."

Wait for explicit confirmation. Iterate on the breakdown if the user requests changes — do not write the task list until confirmed.

> **Capture:** Once confirmed, write the task list to `todo-pr-N.md`.

---

### 7. Finalise documents

By this point the docs have been built up incrementally — fill in any remaining sections and ensure everything is complete and consistent. For **Extend**, **Sibling**, and **New sections** modes, edit existing files rather than overwriting.

#### `todo-pr-N.md`

```markdown
# PR-N Task Breakdown — [work title]

## Tasks

- [ ] **1. [First task]**
  - [ ] 1.1. [Subtask]
  - [ ] 1.2. [Subtask]

- [ ] **2. [Second task]**
  - [ ] 2.1. [Subtask]
```

Keep tasks small and specific. Each task should have a clear "done when" implied by its name. If you have more than ~5 subtasks under one parent, consider splitting the parent.

#### `todo-pr-N_plan.md`

```markdown
# PR-N Plan — [work title]

## Goal / Why
[One paragraph — what this accomplishes and the motivation behind it]

## Features
- [Feature one]
- [Feature two]

## Out of scope
- [What we're not doing]
```

#### `todo-pr-N_architecture.md`

```markdown
# PR-N Architecture — [work title]

## Tech stack
[Languages, frameworks, key libraries — or "not yet determined"]

## Folder structure
[Relevant directories and what lives where — stub if unknown]

## Constraints
[Hard limits: must use X, must not touch Y, performance requirements, etc.]
```

#### `todo-pr-N_notes.md`

```markdown
# PR-N Notes — [work title]

## Doc purpose

| Doc | Purpose |
|-----|---------|
| `_plan` | What and why — goals, features, scope, intent. Written upfront. |
| `_notes` | How — conventions, agreements, decisions that apply across tasks. Stable reference. |
| `_changelog` | What happened — append-only per-task record of decisions, changes, and learnings. |
| `_architecture` | Tech stack, folder structure, hard constraints. |

Read `_notes` to understand the rules. Read `_changelog` to understand why they are the way they are.
```

For **New sections** mode, append a new named section to the relevant existing doc rather than replacing content.

---

### 8. Review

The documents have been maintained throughout the conversation — this step is a review, not a documentation event.

Run `todo-report` to surface the summary view. Then invite the user to check the docs directly if they want to go deeper:

> "The docs are up to date — here's the summary. If anything looks off, open the files directly:
> - `todo-pr-N_plan.md`
> - `todo-pr-N_architecture.md`
> - `todo-pr-N_notes.md`
>
> Ready to start, or anything to adjust?"

---

## Key Principles

- Never overwrite existing content without explicit confirmation
- Small tasks over large ones — clearer, tighter feedback loops
- The `_plan` doc is written upfront and should not change during execution; new scope goes in `_notes`
- When in doubt about task granularity, err smaller — tasks can always be grouped, but coarse tasks are hard to delegate cleanly

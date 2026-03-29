---
name: plan-context
description: Resolve the PR number, check for existing todo files, determine how new work relates to existing work, and create stub planning docs. First phase of the planning conversation. Use at the start of a planning session or when invoked via /plan-context.
allowed-tools: Read, Write, Edit, Glob, Bash
---

# Plan Context

Establish context before the planning conversation begins: resolve the PR number, check for existing work, and create stub docs.

## Steps

### 1. Resolve PR number

If a PR number was passed as an argument, use it — skip branch detection.

Otherwise, run `git branch --show-current` and `gh pr view --json number,title 2>/dev/null`, then **ask the user explicitly**:

| Branch state | What to ask |
|---|---|
| On `main` | "What should we call this work? Do you have a branch or PR in mind, or should we use a placeholder for now?" |
| Feature branch, no PR | "We're on `[branch]` — is this new work related to that branch, or a separate initiative? And do you have a PR number yet?" |
| Feature branch with open PR | "I can see PR #N (`[title]`). Is this new work for that PR, or something separate?" |

Do not assume a PR number or default to a timestamp without asking. If the user doesn't have a PR yet, offer the timestamp placeholder and confirm: "I'll use `todo-pr-YYYYMMDD` as a placeholder — you can rename it once the PR is open. Does that work?"

### 2. Check for existing todo files

Look for `todo-pr-*.md` in `.claude/`. Read the most recently modified one if found.

**If none found:** proceed to step 3.

**If found:** surface a brief summary — PR number, task count, done count — then ask:

> I found an existing todo list (`todo-pr-N.md`, M tasks, X done). How does this new work relate?
>
> 1. **Replace** — archive or discard existing docs and start fresh
> 2. **Extend** — add new tasks to the existing list under the same scope
> 3. **Sibling** — nest existing tasks one level deeper and add this as a peer
> 4. **New sections** — keep the task list as-is but add sections to existing docs

Wait for the user's choice before proceeding.

#### Mode handling

**Replace**
- Ask: "Archive the existing docs (rename with `_archived` suffix), or discard?"
- After confirming, proceed as if starting fresh.

**Extend**
- Ask how the new work relates to the existing scope (one sentence — goes into `_notes`).
- New tasks will be appended continuing from the highest existing task number.

**Sibling**
- Ask: "What should the label be for the existing work as a group?" (e.g. "Phase 1", "Backend")
- Renumber existing tasks one level deeper:
  - Old top-level task `1` with subtasks `1.1`, `1.2` → becomes `1.1` with subtasks `1.1.1`, `1.1.2`
  - Old top-level task `2` with subtask `2.1` → becomes `1.2` with subtask `1.2.1`
- New work becomes task `2` with its own breakdown.

**New sections**
- Ask what the new section is about (one sentence).
- Add a named section to `_notes` and/or `_architecture` as appropriate.
- Skip to plan-delivery (no new plan doc needed unless the user wants one).

### 3. Create stub docs

Create the four planning docs with title + section headers only. This establishes the files early so subsequent phases can append to them incrementally.

- `.claude/todo-pr-N.md`
- `.claude/todo-pr-N_plan.md`
- `.claude/todo-pr-N_architecture.md`
- `.claude/todo-pr-N_notes.md`

For **Extend**, **Sibling**, and **New sections** modes, edit existing files rather than creating new ones.

Confirm: "Docs are ready at `.claude/todo-pr-N*.md`. Let's start with what's driving this work."

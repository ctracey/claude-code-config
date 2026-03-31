---
name: swc_plan-context
description: Confirm the working branch, check for existing workloads, and create stub planning docs. First phase of the planning conversation. Use at the start of a planning session or when invoked via /swc-plan-context.
allowed-tools: Read, Write, Edit, Glob, Bash
---

# Plan Context

Establish context before the planning conversation begins: confirm the branch, check for existing work, and create stub docs.

## Steps

### 1. Confirm the branch

Run `git branch --show-current` to detect the current branch.

Ask the user:

| Branch state | What to ask |
|---|---|
| On `main` | "You're on `main`. Would you like to create a new branch for this work, or track it here?" |
| Feature branch | "You're on `<branch>`. Start the workload here, or switch to a different branch first?" |

If the user wants to switch: help them create or checkout the branch (`git checkout -b <name>` or `git checkout <name>`), then re-run `git branch --show-current`.

Once confirmed, derive the folder name: replace every `/` in the branch name with `_`.

> Example: `feature/swc-refactor` → `feature_swc-refactor`

### 2. Check for an existing workload

Read `.swc/_meta.json` if it exists. Check whether the confirmed branch already has an entry in `workloads`.

**If no entry found:** proceed to step 3.

**If entry found:** read `.swc/<folder>/workload.md` and surface a brief summary — work item count, done count — then ask:

> I found an existing workload for `<branch>` (M work items, X done). How does this new work relate?
>
> 1. **Replace** — archive or discard the existing workload and start fresh
> 2. **Extend** — add new work items continuing from the highest existing number
> 3. **Sibling** — nest existing work items one level deeper and add this as a peer
> 4. **New sections** — keep the workload as-is but add sections to existing docs

Wait for the user's choice before proceeding.

#### Mode handling

**Replace**
- Ask: "Archive the existing docs (rename folder with `_archived` suffix), or discard?"
- Remove or rename the old entry in `_meta.json`. Proceed as if starting fresh.

**Extend**
- Ask how the new work relates to the existing scope (one sentence — goes into `notes.md`).
- New work items will be appended continuing from the highest existing number.

**Sibling**
- Ask: "What should the label be for the existing work as a group?" (e.g. "Phase 1", "Backend")
- Renumber existing work items one level deeper:
  - Old top-level item `1` with sub-items `1.1`, `1.2` → becomes `1.1` with sub-items `1.1.1`, `1.1.2`
  - Old top-level item `2` with sub-item `2.1` → becomes `1.2` with sub-item `1.2.1`
- New work becomes item `2` with its own breakdown.

**New sections**
- Ask what the new section is about (one sentence).
- Add a named section to `notes.md` and/or `architecture.md` as appropriate.
- Skip to `swc-plan-delivery` (no new plan doc needed unless the user wants one).

### 3. Create stub docs

Write or update `.swc/_meta.json`:

```json
{
  "version": 1,
  "workloads": {
    "<branch>": "<folder>"
  }
}
```

Create `.swc/<folder>/` with stub files (title + section headers only):

- `workload.md`
- `plan.md`
- `architecture.md`
- `notes.md`
- `changelog.md`

For **Extend**, **Sibling**, and **New sections** modes, edit existing files rather than creating new ones.

Confirm: "Workload ready at `.swc/<folder>/`. Let's start with what's driving this work."

## Exit criteria

**Done when:**
- Branch confirmed with user
- Existing-work mode chosen (if applicable)
- `_meta.json` written with branch→folder entry
- Stub docs created at `.swc/<folder>/`

**Return control to `swc-begin`.**

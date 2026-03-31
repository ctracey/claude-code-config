---
name: swc_plan-context
description: Confirm the working branch, check for existing workloads, and create stub planning docs. First phase of the planning conversation. Use at the start of a planning session or when invoked via /swc-plan-context.
allowed-tools: Read, Write, Edit, Glob, Bash
---

# Plan Context

Establish context before the planning conversation begins: confirm the branch, check for existing work, and create stub docs.

## Steps

### 1. Resolve the branch and folder

Follow the `swc_resolver --create` skill. It handles:
- Detecting the current branch
- Warning if on `main` and offering to create a feature branch
- Deriving the folder name (`/` → `_`)
- Looking up or scanning for an existing workload
- Creating the folder under `.swc/` and updating `_meta.json`

Once `swc_resolver` completes, you have:
- The confirmed branch name
- The folder path `.swc/<folder>/`
- Whether an existing workload was found

### 2. Handle existing workload (if found)

If `swc_resolver` found an existing workload, read `.swc/<folder>/workload.md` and surface a brief summary — work item count, done count — then ask:

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

Create `.swc/<folder>/` stub files (title + section headers only):

- `workload.md`
- `plan.md`
- `architecture.md`
- `notes.md`
- `changelog.md`

For **Extend**, **Sibling**, and **New sections** modes, edit existing files rather than creating new ones.

Confirm: "Workload ready at `.swc/<folder>/`. Let's start with what's driving this work."

## Exit criteria

**Done when:**
- Branch confirmed with user (via `swc_resolver`)
- Existing-work mode chosen (if applicable)
- `_meta.json` written with branch→folder entry (via `swc_resolver`)
- Stub docs created at `.swc/<folder>/`

**Return control to `swc-begin`.**

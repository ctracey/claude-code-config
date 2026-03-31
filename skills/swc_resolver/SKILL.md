---
name: swc_resolver
description: Resolve or create the active SWC workload folder for the current branch. Single source of truth for branch→folder naming. Use when you need to find the active workload, or when invoked via /swc-resolver.
allowed-tools: Read, Glob, Bash, Write
---

# SWC Resolver

Determine the `.swc/<folder>/` for the current (or specified) branch. In create mode, also creates the folder and updates `_meta.json`.

## Arguments

- `/swc-resolver` — resolve existing workload for the current branch
- `/swc-resolver <branch>` — resolve existing workload for a specific branch
- `/swc-resolver --create` — resolve or create the workload folder for the current branch (used by planning skills)

## Steps

### 1. Determine the branch

Run `git branch --show-current` (or use the branch argument if supplied).

**If the branch is `main` or `master`:**

```
⚠ You're on `main`. Working on a feature branch keeps your work isolated
  and makes it easier to review before merging.

  Create a branch now? (y / n — continue on main):
```

- **Yes:** ask for a branch name, run `git checkout -b <name>`, use the new branch.
- **No:** continue on `main`. No further warning.

### 2. Derive the folder name

Replace every `/` in the branch name with `_`.

> Example: `feature/swc-refactor` → `feature_swc-refactor`

### 3. Try _meta.json

Read `.swc/_meta.json`.

- **Found and branch has an entry:** folder confirmed → skip to step 5.
- **Missing or branch not in map:** print a notice and continue to step 4:
  ```
  Note: .swc/_meta.json not found — falling back to folder scan.
  ```

### 4. Scan folders

List all folders under `.swc/` (exclude `_meta.json`). Compare against the derived folder name from step 2.

**No folders found — resolve mode:**
```
No workload found under .swc/. Run /swc-begin to start one.
```
Stop.

**No folders found — create mode:**
Use the derived folder name from step 2. Proceed to step 5.

**One folder found:** confirm with the user before proceeding, noting whether it matches the branch:
```
Found one workload: .swc/<branch-subfolder>/workload.md
[MATCH] This folder matches your current branch.   ← or [NO MATCH] if it doesn't
Use this? [Y/n]:
```
If the user declines:
- Resolve mode: stop.
- Create mode: use the derived folder name from step 2 and treat as a new workload.

**Multiple folders found:** list them all, flag matches, and ask which to use:
```
Multiple workloads found — which one?
  1. .swc/<branch-subfolder-one>/workload.md  [MATCH]
  2. .swc/<branch-subfolder-two>/workload.md
Enter a number (or 'new' to start a fresh workload):
```

### 5. Persist the mapping to _meta.json

Upsert the branch→folder mapping in `.swc/_meta.json`.

Read the file first if it exists (to preserve other entries):

```json
{
  "workloads": {
    "<branch-name>": "<folder-name>"
  }
}
```

Write the updated file. Print nothing — this is a silent side-effect.

### 6. Create the folder (create mode only)

If in create mode and `.swc/<folder>/` does not yet exist, create the directory by writing a `.swc/<folder>/.gitkeep` placeholder, then immediately delete it — or simply rely on the calling skill to write the first stub file into the folder. Print nothing.

### 7. Return

**Resolve mode:** print the resolved path:
```
Resolved: .swc/<folder>/workload.md
```

**Create mode:** return the resolved folder path to the calling skill. Do not print a confirmation — the calling skill handles that.

> **Goal:** single source of truth for branch→folder naming. Never silently load the wrong workload. When in doubt, ask.

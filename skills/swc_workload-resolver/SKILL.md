---
name: swc_workload-resolver
description: Resolve the active SWC workload folder for the current branch. Returns the path to workload.md. Use when you need to find the active workload, or when the user asks which workload is active, or invokes /swc-workload-resolver.
allowed-tools: Read, Glob, Bash, Write
---

# SWC Workload Resolver

Determine which `.swc/<folder>/workload.md` corresponds to the current (or specified) branch.

## Arguments

- `/swc-workload-resolver` — resolve workload for the current branch
- `/swc-workload-resolver <branch>` — resolve workload for a specific branch

## Steps

### 1. Determine the branch

Run `git branch --show-current` (or use branch argument if supplied).

### 2. Try _meta.json

Read `.swc/_meta.json`.

- If found: look up the branch in the `workloads` map → folder name → **done**, return `.swc/<folder>/workload.md`
- If missing: print a notice and continue to step 3:
  ```
  Note: .swc/_meta.json not found — falling back to folder scan.
  ```

### 3. Fallback — scan folders

List all folders under `.swc/` (exclude `_meta.json`). Folder names use the branch name with `/` replaced by `_` (e.g. `feature/my-work` → `feature_my-work`). Compute the expected folder name from the current branch.

**No folders found:**
```
No workload found under .swc/. Run /swc-begin to start one.
```
Stop.

**One folder found:** confirm with the user before proceeding, noting whether it matches the branch:
```
Note: .swc/_meta.json not found — falling back to folder scan.
Found one workload: .swc/<branch-subfolder>/workload.md
[MATCH] This folder matches your current branch.   ← or [NO MATCH] if it doesn't
Use this? [Y/n]:
```
If the user declines, stop.

**Multiple folders found:** list them all, flag matches, and ask which to use:
```
Note: .swc/_meta.json not found — falling back to folder scan.
Multiple workloads found — which one?
  1. .swc/<branch-subfolder-one>/workload.md  [MATCH]
  2. .swc/<branch-subfolder-two>/workload.md
Enter a number, or type a branch name:
```

### 3.5. Persist the mapping to _meta.json

Once the folder is confirmed (whether via step 2 success or step 3 user choice), write the branch→folder mapping to `.swc/_meta.json`.

Read the file first if it exists (to preserve other entries). Then upsert the current branch:

```json
{
  "workloads": {
    "<branch-name>": "<folder-name>"
  }
}
```

Write the updated file. Print nothing — this is a silent side-effect.

### 4. Return the resolved path

The resolved path is `.swc/<folder>/workload.md`. Pass this to the calling skill or, if invoked standalone, print it:

```
Resolved: .swc/<folder>/workload.md
```

> **Goal:** never silently load the wrong workload. When in doubt, ask.

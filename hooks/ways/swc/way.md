---
description: SWC workload tracking — persistent workload files in .swc/ for multi-session continuity
vocabulary: swc workload work item tracking cross-session multi-session persistent task todo picking resume continuity progress
threshold: 2.0
pattern: swc|workload|work.?item|\.swc
files: \.swc/.*\.md$
scope: agent, subagent
---
# SWC Workload Tracking Way

## Persistent Workload Files

For complex, multi-session work, create a workload under `.swc/`:

```
.swc/
├── _meta.json                     # branch→folder mapping
└── feature_my-work/              # branch name with / → _
    ├── workload.md               # work item list
    ├── plan.md
    ├── architecture.md
    ├── notes.md
    └── changelog.md
```

**When to create:**
- Feature work spanning multiple sessions
- Complex initiatives with multiple work items
- Any work that benefits from a persistent plan

**When to read:**
- At session start, check for an active workload matching the current branch
- Before starting work — check if there's prior context

**Workload folder resolution:**
1. Run `git branch --show-current`
2. Read `.swc/_meta.json`
3. Look up branch in `workloads` map → folder name
4. Fallback: most recently modified folder under `.swc/`

**Work item status markers:**

| Marker | Meaning |
|--------|---------|
| `[ ]`  | Not started |
| `[-]`  | In progress (one or more sub-items started or done, but not all done) |
| `[x]`  | Done (all sub-items complete) |

**Parent work item rules:**
- When sub-items exist, the parent status reflects them: any sub-item in progress → parent is `[-]`; all sub-items done → parent is `[x]`
- If a parent has no sub-items, mark it directly

**Cleanup:**
When all items complete, recommend deleting the workload folder. Git history preserves it. Don't let completed workloads accumulate.

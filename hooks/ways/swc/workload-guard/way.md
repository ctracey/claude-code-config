---
files: workload\.md$
scope: agent, subagent
---

# Workload Edit Guard

**Do not edit `workload.md` directly to change work item status.**

Always use the `swc-workload-update` skill instead:

```
swc-workload-update <item> done        # marks [x], rolls up parent
swc-workload-update <item> in-progress # marks [-], rolls up parent
swc-workload-update <item> reset       # marks [ ], rolls up parent
```

**Why:** `swc-workload-update` handles parent rollup automatically. A direct checkbox edit leaves parent items with stale status markers (`[ ]` instead of `[-]` or `[x]`), which corrupts the workload summary and misleads future sessions.

The only legitimate reason to edit `workload.md` directly is structural changes — adding, removing, or renumbering work items. Status changes always go through `swc-workload-update`.

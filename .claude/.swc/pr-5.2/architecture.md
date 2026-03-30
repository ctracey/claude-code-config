# PR-5.2 Architecture — SWC Refactor

## File System Changes

### Ways

```
hooks/ways/meta/
├── tracking/
│   └── way.md                    RESTORED to pre-PR-5 (simple format, todo- paths)
│   └── naming/                   DELETED (moved to swc/naming/)
│       └── way.md
├── planning/
│   └── way.md                    MOVED → swc/planning/way.md
└── swc/
    ├── way.md                    NEW parent — current tracking/way.md content adapted for swc
    │                             (hierarchical task format, swc- file paths, swc vocabulary)
    ├── naming/
    │   └── way.md                FROM: tracking/naming/way.md (update todo- → swc- throughout)
    └── planning/
        └── way.md                FROM: planning/way.md (update todo-*/swc-* skill refs)
```

The `swc/way.md` parent carries the full SWC tracking format. Children (`naming/`, `planning/`) get 20% threshold lowering once the parent fires.

### Skills

All 19 `skills/todo-*` directories renamed to `skills/swc-*`:

| Old | New |
|-----|-----|
| `skills/todo-begin/` | `skills/swc-begin/` |
| `skills/todo-plan-context/` | `skills/swc-plan-context/` |
| `skills/todo-plan-intent/` | `skills/swc-plan-intent/` |
| `skills/todo-plan-solution/` | `skills/swc-plan-solution/` |
| `skills/todo-plan-delivery/` | `skills/swc-plan-delivery/` |
| `skills/todo-plan-breakdown/` | `skills/swc-plan-breakdown/` |
| `skills/todo-plan-finalise/` | `skills/swc-plan-finalise/` |
| `skills/todo-list/` | `skills/swc-list/` |
| `skills/todo-report/` | `skills/swc-report/` |
| `skills/todo-report-plan/` | `skills/swc-report-plan/` |
| `skills/todo-report-notes/` | `skills/swc-report-notes/` |
| `skills/todo-changelog/` | `skills/swc-changelog/` |
| `skills/todo-update/` | `skills/swc-update/` |
| `skills/todo-execute/` | `skills/swc-execute/` |
| `skills/todo-workflow-orchestrator/` | `skills/swc-workflow-orchestrator/` |
| `skills/todo-workflow-progress/` | `skills/swc-workflow-progress/` |
| `skills/todo-test-workflow/` | `skills/swc-test-workflow/` |
| `skills/todo-test-stage1/` | `skills/swc-test-stage1/` |
| `skills/todo-test-stage2/` | `skills/swc-test-stage2/` |

### Tracking File Convention

```
.claude/.swc/
├── meta.json                        # branch→folder mapping + metadata
└── feature_swc-refactor/            # branch name with / → _
    ├── workload.md                  # work item list (was: task list)
    ├── plan.md
    ├── architecture.md
    ├── notes.md
    └── changelog.md
```

**`meta.json` shape:**
```json
{
  "version": 1,
  "workloads": {
    "feature/swc-refactor": "feature_swc-refactor"
  }
}
```

**Branch confirmation flow** (in `swc-plan-context`):
1. Run `git branch --show-current`
2. Ask: "You're on `<branch>`. Start the workload here, or switch to a new branch first?"
3. If switching: help create/checkout, re-detect, then continue
4. Write entry to `meta.json`, create folder, create stub files

## Cross-Reference Map

Skills with internal references that need updating beyond directory rename:

| Skill | References to update |
|-------|---------------------|
| `swc-begin` | Stage skill names in JSON workflow definition (`todo-plan-*` → `swc-plan-*`) |
| `swc-plan-context` | File paths (`.claude/todo-pr-*.md` → `.claude/.swc/pr-*.md`), slash commands |
| `swc-execute` | File paths (all four doctype variants) |
| `swc-list` | File glob pattern (`.claude/todo-pr-*.md` → `.claude/.swc/pr-*.md`) |
| `swc-report`, `swc-report-*` | File path references |
| `swc-changelog` | File path references |
| `swc-update` | File path references |
| `swc-workflow-orchestrator` | Verify no hardcoded skill names |

## Decisions Made

- `swc/way.md` is the parent way; content = current tracking way (PR-5 version) adapted for swc. No separate `swc/tracking/` subdirectory — the parent IS the tracking way.
- `tracking/naming/` → `swc/naming/` (not `swc/tracking/naming/`).
- `swc/way.md` vocabulary uses `swc` keywords + `\.claude/\.swc/` file pattern. The original `tracking/way.md` retains `todo` vocabulary for the general case.
- Legacy trigger terms (`task`, `todo`, `task list`) are kept in way `vocabulary:` and skill `description:` fields as matching aliases. Claude's output uses swc semantics (workload, work item) regardless of which term triggered the skill.

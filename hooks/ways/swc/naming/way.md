---
description: SWC workload file naming — folder-per-branch structure with plain doc names
vocabulary: swc workload naming convention folder branch doctype plan architecture notes changelog
threshold: 2.0
scope: agent, subagent
---
# SWC Workload File Naming

Workloads live in `.swc/` — one folder per branch.

```
.swc/
├── _meta.json
└── feature_my-work/
    ├── workload.md
    ├── plan.md
    ├── architecture.md
    ├── notes.md
    └── changelog.md
```

## Branch → Folder Mapping

Branch names with `/` are mapped to `_` for filesystem safety:

| Branch | Folder |
|--------|--------|
| `feature/my-work` | `feature_my-work` |
| `fix/auth-bug` | `fix_auth-bug` |
| `main` | `main` |

The canonical mapping lives in `_meta.json`:

```json
{
  "version": 1,
  "workloads": {
    "feature/my-work": "feature_my-work"
  }
}
```

## Doc Files

| File | Purpose | Used by |
|------|---------|---------|
| `workload.md` | Work item list — progress tracking | Main session |
| `plan.md` | **What and why** — goals, features, intent, out of scope | Main session → subagents |
| `architecture.md` | Tech stack, folder structure, hard constraints | Main session → subagents |
| `notes.md` | **Conventions and agreements** — naming, format, decisions that apply across work items. Stable reference. | Any actor |
| `changelog.md` | **What happened** — append-only per-work-item record of decisions, changes, and learnings. | Any actor |

**`plan.md` captures upfront intent.** What are we building and why?

**`notes.md` is a stable reference.** Conventions agreed mid-session that must survive across sessions. Read this to understand the rules.

**`changelog.md` is append-only.** One section per work item, recording what was decided, changed, or learned during implementation.

Changelog entry format:
```markdown
## Work item N.M — Description `YYYY-MM-DD HH:MM`

- Decision or change made
- Why it was made
```

When a new session picks up mid-work, read `notes.md` first, then skim `changelog.md` for recent context.

## Always lowercase

`feature_my-work` not `Feature_My-Work`

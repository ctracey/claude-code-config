---
description: Cross-session work tracking — persistent todo files in .claude/ for multi-session continuity
vocabulary: tracking cross-session multi-session persistent todo picking resume continuity progress
threshold: 2.0
pattern: tracking.?file|cross.?session|multi.?session|picking.?up|\.claude/todo
files: \.claude/todo-.*\.md$
scope: agent, subagent
---
# Work Tracking Way

## Persistent Tracking Files

For complex, multi-session work, create files in `.claude/`:

```
.claude/
├── todo-adr-NNN-description.md   # ADR implementation
├── todo-pr-NNN.md                # PR work/review
├── todo-issue-NNN.md             # Issue resolution
```

**When to create:**
- ADR implementation spanning sessions
- Complex PR with multiple review cycles
- Multi-step issue resolution

**When to read:**
- At session start, check for existing tracking files before beginning work
- Before starting work on an ADR, PR, or issue — check if there's prior context

**Format:**
```markdown
# ADR-081 Implementation: Source Lifecycle

## Tasks

- [-] **1. Parent task**
  - [x] 1.1. Subtask one
  - [ ] 1.2. Subtask two

- [ ] **2. Another parent task**
  - [ ] 2.1. Subtask one
```

**Task status markers:**

| Marker | Meaning |
|--------|---------|
| `[ ]`  | Not started |
| `[-]`  | In progress (one or more subtasks started or done, but not all done) |
| `[x]`  | Done (all subtasks complete) |

**Parent task rules:**
- When subtasks exist, the parent status reflects them: any subtask in progress → parent is `[-]`; all subtasks done → parent is `[x]`
- If a parent has no subtasks, mark it directly

**Cleanup:**
When all items complete, recommend deleting the file. Git history preserves it. Don't let completed files accumulate.

---
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

## Completed
- [x] Phase 1: Pre-ingestion storage
- [x] Phase 2: Offset tracking

## Remaining
- [ ] Phase 3: Deduplication
- [ ] Phase 4: Regeneration
```

**Refactor numbering:** When a refactor task is created while working on a specific task, prefix it with that task number. General refactors unrelated to a specific task stay unprefixed.

```
- [x] 5. Design system
  - [x] 5.R1. Extract data model        # spawned by task 5
  - [x] 5.R2. Phase component           # spawned by task 5
- [ ] R1. General cleanup                # not tied to a task
```

**Cleanup:**
When all items complete, recommend deleting the file. Git history preserves it. Don't let completed files accumulate.

## Related
- Cleanup prompt at commit time → `softwaredev/delivery/commits` (Post-Commit Cleanup)

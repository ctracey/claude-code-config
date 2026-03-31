# PR-5.2 Notes — SWC Refactor

## Key Decisions

**SWC = Sessionless Workload Context**
The feature set name for the planning/tracking/execution workflow built in PR-5.

**No backward compatibility**
Existing `todo-pr-*.md` files on disk are accepted as orphaned. New workloads live under `.claude/.swc/<branch_folder>/`. No migration tooling needed.

**Terminology**
- "task list" → **workload**
- "task" → **work item**
- Use swc semantics in all skill output and user-facing text. Keep legacy terms (`task`, `todo`) only as trigger aliases in `vocabulary:` and `description:` fields so natural language still matches.

**Keyed by branch, not PR**
Workloads are identified by branch name. `swc-plan-context` detects the current branch via `git branch --show-current` and confirms with the user — offering the option to switch branches before starting. Branch names with `/` are mapped to `_` for filesystem safety. `_meta.json` at the `.swc/` root holds the canonical branch→folder mapping.

**Tracking way split: two separate concerns**
The tracking way at `hooks/ways/meta/tracking/` serves the general case (any cross-session file tracking). The new SWC-aware tracking way lives under `hooks/ways/meta/swc/tracking/` and knows about the richer task format and doctype convention.

Concretely:
- `hooks/ways/meta/tracking/way.md` — **restored** to pre-PR-5 state (simple `## Completed`/`## Remaining` format, `todo-` paths)
- `hooks/ways/meta/swc/way.md` — the new format (hierarchical tasks, `[ ]`/`[-]`/`[x]` markers, `.claude/.swc/` paths)
- `hooks/ways/meta/swc/naming/way.md` — doctype suffix convention, `.claude/.swc/` paths

**Planning way also moves**
`hooks/ways/meta/planning/way.md` moves to `hooks/ways/meta/swc/planning/way.md`. The planning conversation way is SWC-specific — it governs the `swc-plan-*` skill sequence.

**Slash commands**
All `/todo-*` slash commands become `/swc-*`. The skill `description:` fields carry the trigger text, so updating the name field and description is sufficient.

## Constraints

- Mechanical rename only — no logic changes to any skill
- Ways frontmatter (`vocabulary`, `pattern`, `files`) must be updated in swc ways to match `.claude/.swc/` paths
- The tracking way at `hooks/ways/meta/tracking/` must remain functionally unchanged from main (it's a general utility, not SWC-specific)

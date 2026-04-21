# PR-5.2: Refactor todo-* to swc-* (Sessionless Workload Context)

## Tasks

- [ ] **1. Create swc way tree**
  - [ ] 1.1. Create `hooks/ways/meta/swc/way.md` — parent way, copy current `tracking/way.md` then adapt (`.swc/` paths, swc vocabulary, workload/work item terminology)
  - [ ] 1.2. Create `hooks/ways/meta/swc/naming/way.md` — copy `tracking/naming/way.md` then update paths and terminology
  - [ ] 1.3. Create `hooks/ways/meta/swc/planning/way.md` — copy `planning/way.md` then update skill name refs todo-* → swc-*

- [ ] **2. Revert tracking way to pre-PR-5 state**
  - [ ] 2.1. Restore `hooks/ways/meta/tracking/way.md` to main-branch content
  - [ ] 2.2. Delete `hooks/ways/meta/tracking/naming/` (now lives under swc/)
  - [ ] 2.3. Delete `hooks/ways/meta/planning/way.md` (now lives under swc/)

- [ ] **3. Rename skill directories**
  - [ ] 3.1. Rename all 19 `skills/todo-*` directories to `skills/swc-*`
  - [ ] 3.2. Update `name:` frontmatter in each SKILL.md
  - [ ] 3.3. Update `description:` fields — slash commands `/todo-*` → `/swc-*`, keep legacy trigger terms (`task`, `todo`, `task list`) as matching aliases

- [ ] **4. Update cross-references inside skills**
  - [ ] 4.1. `swc-begin` — update stage skill names in workflow JSON definition
  - [ ] 4.2. `swc-workflow-orchestrator` — update any skill name references
  - [ ] 4.3. `swc-execute` — update file paths to `.claude/.swc/<folder>/` convention
  - [ ] 4.4. `swc-plan-context` — replace PR number resolution with branch detection + confirmation flow; add `_meta.json` read/write; create folder structure
  - [ ] 4.5. All remaining skills — grep for residual `todo-` path references and fix

- [ ] **5. Terminology pass**
  - [ ] 5.1. Replace "task list" → "workload" in all skill body text
  - [ ] 5.2. Replace "task" → "work item" in all skill body text (excluding trigger alias phrases in descriptions)
  - [ ] 5.3. Confirm no residual `todo-pr-` file paths remain in swc skills

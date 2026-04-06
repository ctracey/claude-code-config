# PR-5 Task Breakdown — Claude Code subagent workflow

## Tasks

- [-] **1. MAIN WORKFLOW**
  - [-] 1.1. **Launch new swc project**
     - [x] 1.1.1. Define reference doc conventions (`todo.md`, `plan.md`, `architecture.md`, structure)
     - [x] 1.1.2. Build the todo-begin skill (scaffold todo + plan + architecture from a documented plan)
     - [x] 1.1.3. Explicitly check and confirm git repo state with user as the first step of planning — branch, remote, open PRs — in a streamlined single exchange before proceeding
     - [x] 1.1.4. Split _notes and _changelog into separate docs
     - [-] 1.1.5. Define when and how notes/docs are updated in the workflow (post-task, post-review, or on handoff)
     - [-] 1.1.6. Switch from todo-pr tracking to .swc workload tracking
         - [x] 1.1.6.1. Decide tracking strategy (branch-only vs always vs never)
         - [ ] 1.1.6.2. Add cleanup step to ship flow if branch-only approach adopted


  - [-] 1.2. **Workload management**
     - [x] 1.2.1. Build the todo-list skill
     - [x] 1.2.2. Build the todo-plan skill
     - [x] 1.2.3. Build the todo-notes skill
     - [x] 1.2.4. Build the todo-changelog skill
     - [x] 1.2.5. Build the todo-report skill
     - [x] 1.2.6. Build the todo-update skill
     - [ ] 1.2.7. Build the todo-add skill
     - [x] 1.2.8. Reconsider gitignore of todo files — decide if todo-pr-N files should be tracked in git
     - [x] 1.2.9. Rename this skillset
     - [x] 1.2.10. Move .swc to root of repo instead of inside .claude and consider tracking decoupling — the `todo-*` prefix ties skills to a specific file format; evaluate whether the skills should be renamed to reflect the workflow concept rather than the storage mechanism, and whether tracking (the todo files) should be decoupled from the workflow skills themselves

      
  - [x] 1.3. **Planning workflow for new project**
     - [x] 1.3.1. Explore Aaron's ADR method and assess fit with this workflow
     - [x] 1.3.2. Streamline repo/branch suggestion in plan-context for new projects — when there is no existing repo or branch, guide the user through creating one as part of the first planning step rather than leaving it as an afterthought
     - [x] 1.3.3 workflow visualisation
         - [x] 1.3.3.1. Visualise workflow progress and active step — show which phase the current session is in (planning, executing, reviewing, shipping) and progress within it
         - [x] 1.3.3.2. Explore a workflow config skill or script — single source of truth for step names used by `todo-begin` and the progress banner, avoiding drift between the `steps=` string and the numbered step list
         - [x] 1.3.3.3. Build an abstracted workflow skill — generic workflow engine that any multi-stage process (planning, executing, reviewing, shipping) can use, rather than hardcoding stage logic in individual skills
         - [x] 1.3.3.4. Improve swc_plan-intent skill


  - [-] 1.4. **Task execution workflow**
     - [-] 1.4.1. Build the implementation-workflow skill
     - [ ] 1.4.2 Build the user-handoff skill, Update execute and handoff skills to include a docs/notes update step
     - [ ] 1.4.3. Define the rich summary artifact format
     - [ ] 1.4.4 Implementation agent reflection
         - [ ] 1.4.4.1. Define what the implementation agent should reflect on after completing a task
         - [ ] 1.4.4.2. Capture reflection output as part of the rich summary artifact
     - [ ] 1.4.5 Agent identity. Explore surfacing a clear agent name visible to the user — e.g. "Planner" or "Implementer" — so the user always knows which agent they are talking to and what its role boundary is
     - [ ] 1.4.6. **Review and fine tune**
         - [ ] 1.4.6.1. Define the structured findings format
         - [ ] 1.4.6.2 Build the review subagent skill


- [ ] **2. Advanced improvements**
  - [ ] 2.2. **Ways hygiene**
  - [ ] 2.2.1. Audit existing ways for content that belongs in a different scope or file — ensure new planning/subagent guidance lives in the right way rather than being patched into unrelated ways
  - [ ] 2.3. **Story mapping**
     - [ ] 2.3.1. Consider story mapping as a complement to task breakdown — logical user journey vs delivery roadmap with outcome/learning milestones
  - [ ] 2.4. **MCP service for todo list management**
     - [-] 2.4.1. Design MCP service interface — expose todo file read/write/query operations as MCP tools so any agent or skill can manage todos via a standard protocol rather than direct file I/O
     - [ ] 2.4.2 Implement MCP service

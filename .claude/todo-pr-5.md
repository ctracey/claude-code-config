# PR-5 Task Breakdown — Claude Code subagent workflow

## Tasks

- [-] **1. Main session flow**
  - [x] 1.1. Define reference doc conventions (`todo.md`, `plan.md`, `architecture.md` structure)
  - [x] 1.2. Build the task-execution skill
  - [ ] 1.3. Build the user-handoff skill
  - [x] 1.4. Explore Aaron's ADR method and assess fit with this workflow

- [-] **2. Todo list management skills**
  - [x] 2.1. Build the todo-list skill
  - [x] 2.2. Build the todo-plan skill
  - [x] 2.3. Build the todo-notes skill
  - [x] 2.4. Build the todo-changelog skill
  - [x] 2.5. Build the todo-report skill
  - [x] 2.6. Build the todo-update skill
  - [ ] 2.7. Build the todo-add skill
  - [x] 2.8. Reconsider gitignore of todo files — decide if todo-pr-N files should be tracked in git

- [-] **3. Todo list creation skills**
  - [-] 3.1. Build the todo-begin skill (scaffold todo + plan + architecture from a documented plan)

- [-] **4. Documentation and notes updates**
  - [x] 4.1. Split _notes and _changelog into separate docs
  - [ ] 4.2. Define when and how notes/docs are updated in the workflow (post-task, post-review, or on handoff)
  - [ ] 4.3. Update todo-execute and handoff skills to include a docs/notes update step

- [ ] **5. Delegate to implementation agent**
  - [ ] 5.1. Define the rich summary artifact format
  - [ ] 5.2. Build the implementation-workflow skill

- [ ] **6. Review and fine tune**
  - [ ] 6.1. Define the structured findings format
  - [ ] 6.2. Build the review subagent skill

- [ ] **7. Todo file lifecycle and cleanup**
  - [ ] 7.1. Decide tracking strategy (branch-only vs always vs never)
  - [ ] 7.2. Add cleanup step to ship flow if branch-only approach adopted

- [ ] **8. Implementation agent reflection**
  - [ ] 8.1. Define what the implementation agent should reflect on after completing a task
  - [ ] 8.2. Capture reflection output as part of the rich summary artifact

- [ ] **9. Story mapping**
  - [ ] 9.1. Consider story mapping as a complement to task breakdown — logical user journey vs delivery roadmap with outcome/learning milestones

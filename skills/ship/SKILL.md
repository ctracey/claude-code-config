---
name: ship
description: Ship current work through the branch → commit → push → PR → merge → cleanup flow. Picks up wherever you are in the cycle. Use when the user says "ship it", "land this", "merge this", or invokes /ship.
allowed-tools: Bash, Read, Grep, Glob
---

# Ship Workflow

Deliver current work to main. Assess the current state and pick up from wherever the user is.

## Arguments

- `/ship` — default flow, adapts review gate to repo flavor
- `/ship full-sail` — skip the review pause (solo/pair repos only; team repos still pause)

## Assess First

Run these in parallel to determine current position in the flow:

```bash
git status --short              # Uncommitted changes?
git branch --show-current       # On main or a feature branch?
git log --oneline main..HEAD    # Commits ahead of main?
git remote show origin 2>&1     # Remote tracking state?
```

Also check repo flavor for review gating:

```bash
# Active contributors in last 90 days
git log --since="90 days ago" --format='%aN' | sort -u | wc -l
```

- **≤2 active**: Solo/pair — lightweight PRs, review pause optional
- **3+ active**: Team — review pause mandatory, suggest reviewers

If the GitHub way has already injected context (look for `**Context**: Team project` or
`**Context**: Solo/pair project` in the conversation), use that instead of re-checking.

## Flow Steps (skip what's already done)

### 1. Branch (if on main with changes)

```bash
git checkout -b <branch-name>
```

Pick a name from the changes: `feature/thing`, `fix/thing`, `refactor/thing`.
If the user provides a name, use it. If changes are already committed on main,
create the branch first, then it carries the commits.

### 2. Commit (if uncommitted changes)

Stage and commit. Follow conventional commit format.
If there are multiple logical changes, make multiple atomic commits.
Ask the user for a commit message direction if the intent isn't clear.

### 3. Push

```bash
git push -u origin <branch>
```

### 4. PR

```bash
gh pr create --title "..." --body "$(cat <<'EOF'
## Summary
...

## Test plan
...
EOF
)"
```

Keep the title under 70 characters. Summary should be 1-3 bullets.
For small/obvious changes, the test plan can be brief.

### 5. Review Gate

**Team repos (3+ active contributors):**
- Always pause here. State the change scope and suggest reviewers.
- Do NOT proceed to merge without explicit user approval.
- Exception: `/ship full-sail` is rejected for team repos — tell the user why.

**Solo/pair repos (≤2 active contributors):**
- **Trivial** (typos, config, single-file): merge directly
- **Small** (1-3 files, clear intent): quick self-review of the diff is enough
- **Significant** (architecture, multi-file, behavioral): suggest the user review
- `/ship full-sail` skips the pause entirely for any change size

State your assessment and let the user decide.

### 6. Merge

```bash
gh pr merge <number> --merge
```

Use `--merge` (not squash or rebase) unless the user prefers otherwise.

### 7. Cleanup

```bash
git checkout main && git pull
```

Git typically prunes the remote tracking ref on pull after merge.
If the local branch lingers:

```bash
git branch -d <branch>
```

## Key Principles

- **Don't ask permission for each step** — assess state, propose the full remaining flow, then execute
- **Pause only at decision points**: commit message wording, PR description, review gate
- **If already mid-flow**, pick up from current state — don't restart
- **One commit is fine** for most changes; only split if there are genuinely separate concerns
- **Repo flavor drives review**, not just change size — a one-line fix in a team repo still pauses

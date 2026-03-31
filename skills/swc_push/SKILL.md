---
name: swc_push
description: Prepare a SWC workload session for commit and push — summarise local changes, update changelog and docs, offer a PR comment, confirm ready to push. Mid-session hygiene before git delivery. Use when the user says "update docs & changelog", "wrap up this session", "prep to commit", "push this", or invokes /swc-push.
allowed-tools: Read, Write, Edit, Glob, Bash
---

# SWC Push

Prepare the session's changes for commit and push. Covers content — what changed and why — not git delivery. After this, the user commits and pushes.

## Steps

### 1. Summarise local changes

Run these in parallel:

```bash
git diff --stat HEAD          # files changed
git diff HEAD                 # full diff
git branch --show-current     # active branch
```

Also read `.swc/_meta.json` to find the active workload folder, then read `workload.md` for task context.

### 2. Present summary to user

Output a brief summary:
- Which files changed and what kind of changes (new skill, fix, refactor, docs)
- Which workload task(s) this relates to, if determinable
- One-line characterisation of the session's intent

Then ask:
> "Does that capture what changed this session? Anything to add or correct before I update the docs?"

Wait for confirmation or corrections.

### 3. Update workload changelog

Append a new session entry to `changelog.md` in the active workload folder:

```markdown
## Session — <short description> `YYYY-MM-DD`

- <bullet per meaningful change>
- Motivation: <why, if not obvious>
```

Date is today. Description is a short phrase (not a sentence). Bullets are factual — what changed and why, not a restatement of file names.

### 4. Update other docs if needed

Check whether any other workload docs need updating:
- `notes.md` — if a decision or convention was settled this session
- `plan.md` — if scope changed or a goal was clarified
- `workload.md` — if task status changed (use `swc_workload-update` for this, never edit directly)

Make only changes that reflect what actually happened. Don't pad.

### 5. PR comment

Check whether an open PR exists for this branch:

```bash
gh pr view --json number,title 2>/dev/null
```

If a PR exists, ask:
> "Want to add a comment to the PR summarising this session's changes?"

If yes, draft a short comment (3–5 bullets, no preamble) and post it:

```bash
gh pr comment <number> --body "$(cat <<'EOF'
<session summary bullets>
EOF
)"
```

If no PR exists, or user says no, skip silently.

### 6. Confirm ready

Show the user what was written, then say:
> "Docs updated. Ready to commit and push."

Stop here. Git delivery is the user's next step — they can commit manually or say "commit and push".

## Key principles

- This skill covers content, not git. Do not commit, push, or create PRs.
- Changelog entries are session-level — one entry per session, even if multiple tasks touched.
- PR comment is optional and user-confirmed — never post without asking.
- If no workload is active, write the changelog entry to the most recently modified `.swc/` folder and note it.

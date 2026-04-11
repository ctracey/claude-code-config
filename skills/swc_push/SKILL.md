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

### 5. Confirm ready to commit and push

Show the user what was written to the docs, then ask:
> "Docs updated. Ready to commit and push?"

Wait for confirmation. If they say no or want to make changes, address their feedback and re-confirm before proceeding.

### 6. PR comment

Once the user has confirmed they're ready to commit and push, check for a remote and open PR:

```bash
git remote get-url origin 2>/dev/null
gh pr view --json number,title 2>/dev/null
```

If no remote is configured, ask:
> "No remote configured — would you like to create one (e.g. a GitHub repo with a PR), or keep this local for now?"

- If they want a remote: help them create one and continue to the PR check.
- If they want to stay local: acknowledge briefly and skip the rest of this step.

If a PR exists, draft a short comment (3–5 bullets, no preamble) and show it to the user:
> "Here's a draft PR comment — want me to post it?
>
> [draft comment]"

If yes, post it:

```bash
gh pr comment <number> --body "$(cat <<'EOF'
<draft comment>
EOF
)"
```

If no PR exists, or user says no, skip silently.

Stop here. The user does the actual commit and push.

## Key principles

- This skill covers content, not git. Do not commit, push, or create PRs.
- Changelog entries are session-level — one entry per session, even if multiple tasks touched.
- PR comment is always drafted after the user confirms ready to commit and push — never before.
- PR comment is optional and user-confirmed — never post without showing the draft and getting approval.
- If no workload is active, write the changelog entry to the most recently modified `.swc/` folder and note it.

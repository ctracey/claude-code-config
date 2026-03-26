---
name: todo-changelog
description: Summarise the changelog for the active todo. Use when the user says "what changed", "show me the changelog", "what happened in task N", "recent changes", or invokes /todo-changelog.
allowed-tools: Read, Glob
---

# Todo Changelog

Read the active changelog file and present a concise overview of recent task entries.

## Steps

### 1. Resolve the file

- If PR number supplied, read `.claude/todo-pr-N_changelog.md`
- Otherwise, find the most recently modified `.claude/todo-pr-*.md` in `.claude/` and derive the changelog file from it

### 2. Summarise

- By default, show the **last 3 task entries** — most recent work first
- If the user asked about a specific task (e.g. "what happened in task 2.1"), show that entry in full
- For each entry: include the timestamp from the heading (`YYYY-MM-DD HH:MM`), then one line summary of what was decided or changed
- Entries are in chronological order — do not sort by task number when appending

### 3. Close with an invitation

End with:

> "Say 'show full changelog' to see all entries, or ask about a specific task."

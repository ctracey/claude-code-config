---
name: todo-notes
description: Summarise the notes and decisions captured for the active todo. Use when the user says "what notes do we have", "show me the notes", "what decisions have been made", "summarise the notes", or invokes /todo-notes.
allowed-tools: Read, Glob
---

# Todo Notes

Read the active notes file and present a concise overview of key decisions and conventions captured.

## Steps

### 1. Resolve the file

- If PR number supplied, read `.claude/todo-pr-N_notes.md`
- Otherwise, find the most recently modified `.claude/todo-pr-*.md` in `.claude/` and derive the notes file from it

### 2. Summarise

Open with two lines:
```
NOTES SUMMARY
notes doc: .claude/todo-pr-N_notes.md
```

List topics as a flat bullet list — one line per topic, naming the key decision or convention only. No sub-bullets unless a topic has more than one distinct rule. Keep the whole output under 10 lines.

### 3. Close with an invitation

End with:

> "Ask me about any of these, or say 'show notes doc' to see the complete notes file."

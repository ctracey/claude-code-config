---
name: todo-plan
description: Summarise the plan for the active todo. Use when the user says "what's the plan", "show me the plan", "summarise the plan", or invokes /todo-plan.
allowed-tools: Read, Glob
---

# Todo Plan

Read the active plan file and present a concise high-level summary, leaving the door open for follow-up questions.

## Steps

### 1. Resolve the file

- If PR number supplied, read `.claude/todo-pr-N_plan.md`
- Otherwise, find the most recently modified `.claude/todo-pr-*.md` in `.claude/` and derive the plan file from it

### 2. Summarise

Open with two lines:
```
PLAN SUMMARY
plan doc: .claude/todo-pr-N_plan.md
```

Then three things only:
- **Goal** — one sentence
- **Key features** — 3–5 bullets, one line each
- **Principles** — 2–3 bullets max, only if meaningfully distinct from features

Skip out-of-scope unless the user asks. Do not output the raw plan file.

### 3. Close with an invitation

End with:

> "Ask me about any part, or say 'show plan doc' for the full document."

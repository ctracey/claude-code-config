---
name: todo-report
description: Full status report combining plan summary, task list, and notes overview. Use when the user says "give me a report", "catch me up", "where were we", "status report", "picking up where we left off", or invokes /todo-report.
allowed-tools: Read, Glob
---

# Todo Report

Delegate to the three component skills in order, then add a NEXT STEP section.

1. Invoke `todo-plan`
2. Invoke `todo-list`
3. Invoke `todo-notes`

## 4. NEXT STEP

After the three sections, output:

```
NEXT STEP
[task number] — [one-line description of what this task is about]
```

Identify the first task (or subtask) with status `[ ]` not started, reading the todo file top to bottom. Use the task number and a concise description of its purpose — do not copy the raw task text verbatim if it is verbose.

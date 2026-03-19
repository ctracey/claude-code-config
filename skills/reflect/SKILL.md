---
name: reflect
description: "This skill should be used when the user says 'let's reflect', 'what friction did we hit', 'what worked well', 'what could we improve', or invokes /reflect. Performs a two-phase session review: observe what worked and what had friction, then propose concrete improvements to extensible mechanisms (ways, hooks, skills, settings, memory)."
---

# Session Reflection

Review the current session for patterns worth reinforcing and friction worth resolving. Two phases: observe, then strengthen.

## Phase 1: Observe

Scan the conversation history for signals. Present observations conversationally — not a checklist, but a candid read of how the session went.

### What worked well

Identify patterns, approaches, or decisions that landed smoothly:
- Approaches that produced good results without correction
- Decisions the user confirmed or built on
- Collaboration patterns that felt efficient
- New patterns emerging that could be codified

### What had friction

Identify moments where the session stumbled or could have gone better:
- Corrections — "no, we do it this way"
- Wrong defaults — chose the wrong mechanism, format, or approach
- Repeated guidance — user had to say the same thing twice
- Missed context — information that was available but not leveraged
- Things we knew about but didn't apply strongly enough

### What we knew but underlevered

Identify existing guidance (ways, skills, conventions) that could have prevented friction if applied earlier or more consistently.

**Pause here.** Present Phase 1 observations and wait for the user to react before proceeding. The user may add observations, correct misreadings, or reprioritize.

## Phase 2: Strengthen

For each observation worth acting on, propose a concrete improvement. Match the mechanism to the problem:

| Mechanism | When to use |
|-----------|-------------|
| **New/updated way** | Repeatable workflow convention with a clear trigger context |
| **New/updated hook** | Automated behavior on a specific event (pre-commit, file save, etc.) |
| **New/updated skill** | Reusable command the user invokes on demand |
| **Memory** | User context or preference that informs judgment (no trigger) |
| **Settings change** | Permissions, config, environment variables |
| **Nothing** | One-off situation, already covered, or not worth the overhead |

### Proposal format

For each suggestion, state:
1. **The observation** it addresses
2. **The mechanism** and why it fits
3. **Where it lives** (file path or setting)
4. **Scope** — global (all projects) or project-local, with reasoning
5. **Draft content** or description of the change

Default to generic/global unless the observation is clearly project-specific.

### The genericity test

Before proposing, ask: "Would this apply to a different project with a different stack?" If yes, keep it generic. Avoid coupling suggestions to the current project's language, framework, or tooling unless the observation is inherently project-specific.

## Principles

- **Conversational, not ceremonial** — observations should read like a colleague's honest debrief, not a report
- **Propose, don't act** — present suggestions and let the user decide what's worth keeping
- **Two-phase pause** — always wait for user input between observe and strengthen
- **This session only** — don't reference other sessions or speculate about past work
- **Bias toward strengthening existing mechanisms** over creating new ones

---
name: think
description: Strategy selector for structured reasoning. Lists available thinking frameworks and invokes the right one. Use when facing complex decisions, trade-offs, or when stuck.
allowed-tools: Bash, Read, Glob
---

# Think Strategies

Select and invoke a structured reasoning framework. Each strategy is a separate skill with step-by-step stages.

## Usage

```
/think                    # Show available strategies
/think <strategy>         # Invoke a specific strategy skill
```

## Available Strategies

| Problem Shape | Skill | Stages | When to Use |
|---|---|---|---|
| Multiple viable approaches | `/think-tree` | 7 | "What are the options?" — branch, evaluate, prune |
| Three competing objectives | `/think-trilemma` | 6 | "We can't have all three" — satisfice |
| High-stakes decision | `/think-consistency` | 5 | "Are we sure?" — independent paths, consensus |
| Stuck / need principles | `/think-stepback` | 5 | "Why does this work?" — abstract, then apply |
| Investigation / debugging | `/think-react` | 7 | "Figure out why" — reason-act-observe cycle |

## How It Works

1. The Think Strategies way fires on reasoning-related prompts (via semantic matching)
2. You (or the user) select the appropriate strategy
3. Invoke the strategy skill — it provides step-by-step guidance
4. Follow all stages in order before concluding

Strategy definitions live in `~/.claude/hooks/ways/meta/think/strategies/`.

## Session Lifecycle

Think sessions have a lifecycle: **start → work stages → complete (or abandon)**.

**Before starting a new session:**

```bash
# Check for active think session
cat /tmp/.claude-think-session 2>/dev/null
```

- If a session is active, ask the user: finish it or abandon it first
- Do NOT start a new think session while one is active

**Abandoning a session** (user says "never mind", "skip it", changes topic):

```bash
rm -f /tmp/.claude-think-session /tmp/.claude-way-meta-think-*"${CLAUDE_SESSION_ID:+-$CLAUDE_SESSION_ID}" 2>/dev/null
```

After completion or abandonment, the think way can fire again for new problems.

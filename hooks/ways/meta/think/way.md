---
description: structured reasoning, thinking frameworks, cognitive scaffolding for complex decisions
vocabulary: explore options approaches trade-off balance alternatives stuck principle abstract reasoning framework systematic
threshold: 2.0
scope: agent, subagent
---
# Structured Thinking

When you encounter complexity, don't reach for a framework first. Evaluate whether you need one.

## The Metacognitive Check

Before solving, pause and assess: **is your understanding trending toward clarity or away from it?**

Do not attempt to solve in this first cycle. Just evaluate the direction:

1. **Trending clear** — You can see the shape of the answer. Proceed normally. No scaffolding needed.
2. **Trending unclear** — The problem has competing concerns, hidden dependencies, or you're uncertain which direction to go. Escalate.

## Escalation Gradient

| Level | What happens | When |
|---|---|---|
| **Internal reasoning** | Think harder silently — extend your reasoning, consider more angles | Unclear but likely resolvable with more thought |
| **External strategy** | Use a structured strategy (below) — surfaces your reasoning step-by-step | Internal reasoning isn't converging; the human should see the work |
| **Collaborative** | Discuss with the human — they have context you lack | Strategy hits unknowns that tools can't resolve |

Most problems resolve at level 1. The strategies exist for when they don't.

## External Strategies

**When you decide to escalate, act immediately.** Invoke the skill — don't announce your intention, don't ask permission, don't hedge with "I might want to use..." The decision to escalate IS the decision to act. The human cannot follow your reasoning speed; by the time they'd read a proposal to use a strategy, you should already be working through it.

| Problem Shape | Strategy | Invoke |
|---|---|---|
| Multiple viable approaches | Tree of Thoughts | `/think-tree` |
| Three competing objectives | Trilemma | `/think-trilemma` |
| High-stakes, need confidence | Self-Consistency | `/think-consistency` |
| Stuck, need first principles | Step-Back | `/think-stepback` |
| Investigation or debugging | ReAct | `/think-react` |

Each strategy is a step-by-step scaffold that surfaces your reasoning visibly. If during a strategy you encounter unknowns that your tools can't resolve, the remaining resource is the human — ask them directly.

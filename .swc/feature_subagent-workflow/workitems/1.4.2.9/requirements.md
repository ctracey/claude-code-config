# Requirements — 1.4.2.9: Add quality baseline stage to deliver workflow

## Intent

Add a **quality baseline** stage to the deliver workflow between solution-design and implement. The stage runs health checks scoped to the work area, surfaces findings to the user, and captures the resulting decisions in a `quality-baseline.md` doc. That doc is included in the implementation agent brief so the agent starts with full context on known failures and agreed handling — it never has to make judgement calls about pre-existing issues.

The motivation is to prevent the implementation agent from wasting passes diagnosing failures it didn't cause, and to give the user visibility and control over quality issues before committing to implementation.

## Constraints

- The stage runs in the main session — a human is present for all decisions
- The agent must never encounter an undisclosed pre-existing failure; all known issues and decisions must be in the brief
- `quality-baseline.md` is written by this stage and read by the brief assembler (`swc_workflow_deliver-implement`)
- Health check commands should be sourced from `architecture.md` (agreed test harness) and/or `solution.md` (implementation-specific checks); do not guess

## Out of scope

- Fixing failures (that is a separate work item or a pre-condition task — decided interactively with the user)
- Running the full test suite for unrelated areas; checks should be scoped to the work item area
- Updating the brief assembler to include `quality-baseline.md` (covered by 1.4.2.3)

## Approach direction

Add `quality-baseline` as a new stage skill (`swc_workflow_deliver-quality-baseline`) and insert it into the stage list in `swc_workflow_deliver` between `solution-design` and `implement`. The stage skill runs health checks, presents findings with relevance assessment, and writes `quality-baseline.md` to `.swc/<folder>/workitems/<N>/`.

## Decision log

- **Baseline moved to deliver workflow (not implementation workflow):** a human needs to be present when failures are found so that scope decisions can be made interactively. The implementation agent cannot make these calls.
- **Orient before baseline (in implementation workflow):** originally considered baseline-first for fail-fast; moved to orient-first so the agent understands scope before running checks, enabling relevance assessment. This now applies to the deliver workflow stage — it runs after solution-design so findings can be assessed against the agreed approach.
- **Never fix silently:** if a failure is in scope, the decision is stop (fix first as a separate task) or proceed-with-flag (user accepts the risk). The agent never patches pre-existing failures without explicit instruction.
- **Commands recorded in `quality-baseline.md`:** so the implementation agent can re-run the same checks during its pass to verify it hasn't introduced new failures.

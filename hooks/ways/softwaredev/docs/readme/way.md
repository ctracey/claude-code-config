---
description: README authoring, project overview, getting started guide, README structure
vocabulary: readme project overview getting started quick start onboarding introduction about what is this
threshold: 2.0
files: README\.md$
scope: agent, subagent
---
# README Way

## Philosophy

**Gist first.** A reader should understand what this is, who it's for, and why it exists within 30 seconds.

**Scale to complexity.** Simple project = simple README. Complex project = README + docs tree.

**Progressive disclosure.** README is the front door. Depth lives in `docs/`.

## Anti-Patterns

- **Monolith** — Everything in one massive file
- **Installation-first** — Burying the "what" under "how to install"
- **No context** — Assuming the reader knows what problem this solves
- **Over-documenting simple things** — 500 lines for a utility script

## Standard Structure

Each section below maps to a doc reference where depth is needed.

```markdown
# Project Name

> One sentence: what it is and who it's for.

One paragraph: what problem it solves and why it exists. Answer: what, who, why.

## Architecture

High-level concept — key components and how they relate.
For depth: [docs/architecture.md](docs/architecture.md)

## Environment Setup

Prerequisites and quick start.
For full install and configuration: [docs/environment.md](docs/environment.md)

## Operations

Common commands:
- Run tests: `<command>`
- Start dev server: `<command>`
- Build distribution: `<command>`

For full operational procedures: [docs/operations.md](docs/operations.md)

## CI/CD

Brief description of the pipeline — what triggers it, what it does, where to find it.

## Contributing

How to raise issues, submit PRs, and what the review process looks like.
Link to CONTRIBUTING.md if it exists.

## License

License name + one-line summary. Link to LICENSE file.
```

## When to Use docs/

| Complexity | Documentation |
|------------|---------------|
| Script/utility | README only |
| Small library | README + examples |
| Application | README + `docs/` tree |
| Platform | README + `docs/` + guides + API docs |

## docs/ Structure (when needed)

```
docs/
├── environment.md      # install, config, quick start
├── operations.md       # tests, dev server, build, deploy
├── architecture.md     # design, components, decisions
└── architecture/       # ADRs if using ADR tooling
```

## What the Agent Should Update

When implementing a work item, update README sections that are directly affected:
- New components or concepts → Architecture section
- New commands or changed invocations → Operations section
- Pipeline changes → CI/CD section

Do not rewrite sections unrelated to the work item.

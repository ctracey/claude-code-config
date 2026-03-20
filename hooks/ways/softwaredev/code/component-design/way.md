---
description: Component-first design for React projects — data separation, reusable components, avoid hardcoded content
vocabulary: component react jsx props render data driven reusable extract section content map
threshold: 2.0
pattern: \.(jsx|tsx)$|component|React|useState|useEffect
files: \.(jsx|tsx)$
scope: agent, subagent
---
# Component-First Design

Design with reusable components and data separation from the start. Do not hardcode content into JSX and plan to refactor later.

## Rules

| When | Do |
|------|----|
| Adding repeated or structured content | Define as structured data first, then render with a component |
| Creating a new visual pattern | Extract a reusable component immediately — don't inline |
| Content repeats with variation | Data array + map, not copy-pasted JSX blocks |
| Adding a new section type | Create a component that accepts props, not a one-off JSX block |
| Adding styles for a component | Put them in the component's own CSS file, not a shared/global stylesheet |

## Architecture Pattern

```
src/
  components/
    ComponentName/
      ComponentName.jsx   → Component logic and markup
      ComponentName.css   → Component-scoped styles
      index.js            → Re-export for clean imports
  data/                   → Structured content (arrays, objects, metadata)
  App.jsx                 → Thin shell that composes components with data

test/
  components/
    ComponentName/
      ComponentName.test.jsx   → Mirrors src/ structure
  App.test.jsx
```

Source and test code live in separate trees. The `test/` directory mirrors the `src/` structure so every source file has a predictable test location. No test files inside `src/`.

## Why

Hardcoding content as raw JSX creates unmaintainable files that grow linearly with content. Data-driven components keep content editable independently of presentation and make the UI composable and testable.

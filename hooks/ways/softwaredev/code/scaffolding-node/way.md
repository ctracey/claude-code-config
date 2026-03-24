---
description: Node/React project structure — file placement, test layout, and structural review after changes
vocabulary: scaffold node react vite next component hook util service layout route page structure review improve improvements reorganize move rename misplaced placement what we have
pattern: scaffold|init.*project|create.*component|new.*file|project.?structure|file.?structure|review.*(structure|placement|layout|what we have|improvements)|reorganiz|misplaced
threshold: 2.0
scope: agent, subagent
---
# Node/React Scaffolding

Source in `src/`. Tests mirror under `test/`. No test files inside `src/`.

## File Placement

| Type | Location |
|------|----------|
| Components | `src/components/ComponentName.tsx` |
| Hooks | `src/hooks/useName.ts` |
| Utilities | `src/utils/name.ts` |
| Services | `src/services/name.ts` |
| Types | `src/types/name.ts` |

## Test Placement

Mirror `src/` structure under `test/`:

```
src/components/Header.tsx  →  test/components/Header.test.tsx
src/hooks/useAuth.ts       →  test/hooks/useAuth.test.ts
src/utils/format.ts        →  test/utils/format.test.ts
```

Use Vitest + React Testing Library. Fixtures and test helpers live in `test/`.

## Why

Predictable location for every file type. Anyone finds tests instantly by swapping `src/` for `test/`.

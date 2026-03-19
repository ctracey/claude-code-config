---
description: Keep source and test code in separate directory trees with mirrored structure
vocabulary: test spec unit integration structure directory mirror layout folder organize
threshold: 2.0
pattern: test|spec|\.test\.|\.spec\.|__tests__
files: \.(test|spec)\.(js|jsx|ts|tsx|py|rs|go)$
scope: agent, subagent
---
# Test Structure

Source code lives in `src/` (or equivalent). Test code lives in a separate `test/` tree that mirrors the source structure. No test files inside `src/`.

## Rules

| When | Do |
|------|----|
| Creating a new source file | Place its test at the mirrored path under `test/` |
| Moving or renaming source files | Move the corresponding test to match |
| Adding a new directory under `src/` | Mirror it under `test/` when tests are added |

## Example

```
src/
  utils/
    format.js
  components/
    Header/
      Header.jsx

test/
  utils/
    format.test.js
  components/
    Header/
      Header.test.jsx
```

## Why

Mixing test files into source directories clutters imports, bloats builds (requiring test exclusions), and makes it harder to reason about what ships vs. what verifies. A mirrored tree keeps both concerns clean and every test location predictable.

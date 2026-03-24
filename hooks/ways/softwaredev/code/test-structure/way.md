---
description: Test placement conventions per language ecosystem — ensures tests land in the right location during implementation, quality reviews, refactoring, and scaffolding
vocabulary: test tree mirror directory layout placement co-locate colocate separate test path test directory fixture conftest testutil helper monorepo test organization scaffold project structure file structure code quality review refactor improve clean up
threshold: 1.5
pattern: test|spec|\.test\.|\.spec\.|__tests__|scaffold|project.?structure|file.?structure|code.?quality|review.?(code|quality)|refactor
files: \.(test|spec)\.(js|jsx|ts|tsx|py|rs|go)$|Test\.java$|_spec\.rb$|_test\.rb$|Test\.php$|_test\.exs$|Tests\.cs$|_test\.go$
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

## Language Conventions

### JS / TS — Separate mirrored tree

```
src/utils/format.ts        →  test/utils/format.test.ts
src/components/Header.tsx   →  test/components/Header.test.tsx
```

### Java — Maven / Gradle convention

```
src/main/java/com/app/Service.java  →  src/test/java/com/app/ServiceTest.java
```

### Ruby — RSpec / Minitest

```
lib/models/user.rb     →  spec/models/user_spec.rb
app/services/billing.rb →  test/services/billing_test.rb
```

### PHP — PHPUnit

```
src/Service/Payment.php  →  tests/Service/PaymentTest.php
```

### C# / .NET — Separate test project

```
MyApp/Services/OrderService.cs  →  MyApp.Tests/Services/OrderServiceTests.cs
```

### Elixir — test/ directory

```
lib/my_app/parser.ex  →  test/my_app/parser_test.exs
```

### Python — tests/ directory (pytest)

```
src/myapp/parser.py    →  tests/test_parser.py
src/myapp/utils/fmt.py →  tests/utils/test_fmt.py
```

Fixtures and conftest files live in `tests/` alongside the tests that use them.

### Go — Co-located by convention

Test files live next to the source file in the same package. Do not create a separate test tree.

```
pkg/auth/token.go       →  pkg/auth/token_test.go
internal/parser/parse.go →  internal/parser/parse_test.go
```

Test helpers go in a `testutil/` or `internal/testutil/` package.

### Rust — Inline unit tests, separate integration tests

Unit tests are inline `#[cfg(test)]` modules within the source file. Integration tests go in a top-level `tests/` directory.

```
src/parser.rs           →  unit tests: #[cfg(test)] mod tests { ... } inside parser.rs
                            integration tests: tests/parser_integration.rs
```

## Principle

Tests should be predictably located following the ecosystem's convention. For languages that separate test trees, mirror the source structure. For languages that co-locate (Go, Rust unit tests), keep tests next to the code they verify. The goal is the same: anyone on the team can find a file's tests instantly.

## Why

Mixing test files into source directories clutters imports, bloats builds (requiring test exclusions), and makes it harder to reason about what ships vs. what verifies — in ecosystems where separation is the norm. In ecosystems where co-location is idiomatic, a separate tree fights tooling and conventions for no benefit.

## Related
- Test placement checked during quality reviews → `softwaredev/code/quality`

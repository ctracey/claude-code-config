---
description: Place tests predictably per language convention — mirrored trees, co-location, or inline modules
vocabulary: test spec unit integration structure directory mirror layout folder organize pytest rspec phpunit _test.go cfg test fixture conftest testutil scaffold boilerplate project setup init template refactor reorganize restructure clean up maturity where should tree monorepo testing strategy test plan coverage test placement pipeline ci github actions test runner tsconfig jest config pytest.ini build exclude onboard walkthrough codebase tour source java php rust code package
threshold: 1.5
pattern: test|spec|\.test\.|\.spec\.|__tests__|scaffold|structure|refactor
files: \.(test|spec)\.(js|jsx|ts|tsx|py|rs|go)$|Test\.java$|_spec\.rb$|_test\.rb$|Test\.php$|_test\.exs$|Tests\.cs$
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

---
name: coding
description: Enforces coding best practices: deep-read before modify, LSP-first navigation, TDD red-green-refactor, minimal changes, self-review, verification. Trigger when writing code, modifying source files, fixing bugs, refactoring, or when user says 'implement', 'fix this', 'add feature', 'refactor', 'modify', 'update code'.
---

# Coding — Write Code Right

**Core principle:** Every code change must be minimal, tested, verified, and self-reviewed before claiming done.

## Phase 0.5: Deep Read (MANDATORY)

> The most expensive failure mode is code that is "correct in isolation but breaks the surrounding system".

Before writing any code:

1. `goto_definition` — navigate to the code you'll change, read it deeply
2. `find_references` — map ALL callers and dependents of symbols you'll modify
3. `get_document_symbols` — understand internal structure of files you'll modify
4. Read adjacent code — naming conventions, error handling patterns, test patterns
5. `get_diagnostics` — record current state (zero new errors allowed after your change)
6. Produce a **Codebase Understanding** summary:

```
Codebase Understanding:
- Module role: [what this module does]
- Callers: [who calls the code you'll modify]
- Dependencies: [what the target code depends on]
- Conventions: [code style, naming, error handling patterns]
- Impact scope: [which other files/modules could be affected]
```

**Iron Rules — no exceptions:**
- No modify without `goto_definition`
- No refactor without `find_references`
- No new public API without searching for existing similar abstractions
- Match existing code style — don't introduce new conventions

## Phase 1: TDD — Red → Green → Refactor

1. Write failing test FIRST → run → must FAIL (red)
2. Write minimal implementation to pass the test
3. Run test → must PASS (green)
4. Refactor only when duplication is real, run tests again

**Minimal Change Rules:**
- Single responsibility — does this change do exactly one thing?
- No drive-by fixes — unrelated improvements go in separate commits
- No new dependencies unless essential
- Don't "fix" old code outside your change scope

## Phase 2: Self-Verify

1. Run full test suite — must show 0 new failures
2. `get_diagnostics` on all modified files — must show 0 new errors
3. `git diff --stat` — every changed file must be intentional
4. Regression check: revert fix → test must FAIL → restore → test must PASS

## Phase 3: Self-Review

Before committing, review your own diff:

- SRP — one reason to change?
- No dead code introduced
- Error paths handled
- Boundary conditions (null, empty, zero, max)
- No hardcoded values

Then explain your changes:
1. What did I change and why?
2. How do changes interact with callers/dependencies from Phase 0.5?
3. Any potential side effects?

If you discover a contradiction while explaining → go back and fix before committing.

## Phase 4: Commit

```bash
git add -p  # stage intentionally, never git add .
git commit -m "<type>: <what changed and why>"
```

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`

## Rust Specifics

- No `unwrap()` in production code — use `?`, `.expect("reason")`, or proper error handling
- Prefer `&str` over `String` in function parameters where possible
- Handle all `Result`/`Option` paths — no silent discards
- After trait/struct changes: `cargo check` across the workspace

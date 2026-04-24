---
name: unit-test
description: Use when generating code, adding features, fixing bugs, or refactoring — ensures unit tests are written alongside production code following project conventions
---

# Unit Test Generation

**Core principle:** Every code change ships with tests that prove it works. No code without a covering test.

## When to Use

- Writing new functions, methods, structs, or trait implementations
- Fixing a bug (test must reproduce the bug first)
- Refactoring existing code (tests must exist before changing behavior)
- Adding a new endpoint, service method, or repository function
- When `cargo test` has no coverage for the module being changed

## Project Conventions

### Test Module Structure

Always use inline test modules — no separate test files:

```rust
// At the bottom of the source file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // arrange
        let input = create_test_input();

        // act
        let result = function_under_test(input);

        // assert
        assert_eq!(result, expected);
    }
}
```

### Naming

Use `test_<what>_<condition>` pattern:

```rust
#[test]
fn test_parse_valid_email() { }

#[test]
fn test_parse_empty_string_returns_error() { }

#[test]
fn test_hash_password_empty_input_fails() { }
```

### Async Tests

Use `#[tokio::test]` for any async code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_user_returns_none_for_missing_id() {
        let service = setup_test_service();
        let result = service.find_user(Uuid::new_v4()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
```

### Assertions

| Pattern | Use |
|---------|-----|
| `assert_eq!(actual, expected)` | Value equality |
| `assert!(condition, "message")` | Boolean with context |
| `assert!(matches!(val, Pattern))` | Enum variant matching |
| `assert!(result.is_ok())` | Result success |
| `assert!(result.is_err())` | Result failure |
| `#[should_panic(expected = "msg")]` | Expected panics |

### Mocking Dependencies

Use manual mock structs matching project style:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockRepo;

    #[async_trait]
    impl UserRepository for MockRepo {
        async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, Error> {
            Ok(Some(User { id: Uuid::new_v4(), name: "Test".into() }))
        }
    }

    #[tokio::test]
    async fn test_service_with_mock() {
        let service = UserService::new(MockRepo);
        let user = service.get_user(Uuid::new_v4()).await.unwrap();
        assert_eq!(user.unwrap().name, "Test");
    }
}
```

For simple cases, implement the trait on `()`:

```rust
impl TraitName for () {
    fn method(&self) -> Result<(), Error> { Ok(()) }
}
```

## Test Checklist

For every code change, verify:

- [ ] Happy path tested
- [ ] Error/failure paths tested
- [ ] Edge cases: empty input, zero, None, boundary values
- [ ] No `unwrap()` in production code (use `?` or `.expect("reason")`)
- [ ] Tests use `unwrap()` freely — panics are the assertion mechanism
- [ ] `cargo test -p <crate>` passes with 0 failures

## What to Test per Layer

| Layer | Test Focus |
|-------|------------|
| `model/` | Serialization, validation, `From`/`Into` impls, default values |
| `service/` | Business logic with mocked repos, error propagation, edge cases |
| `repo/` | Query construction (mock DB connection), error mapping |
| `entities/` | Trait method behavior, CRUD logic |
| `apis/` | Handler returns correct status codes, response shapes |
| `stream/` | Message parsing, handler dispatch logic |

## Running Tests

```bash
# Single crate
cargo test -p features-auth-service

# Whole workspace
cargo test --workspace

# Specific test
cargo test -p features-auth-service test_hash_password
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Testing implementation details | Test behavior and outputs, not internal state |
| No error path tests | Add tests for every `Err` and `None` branch |
| Shared mutable state between tests | Each test creates its own data |
| Missing `#[cfg(test)]` on test module | Tests compile into production binary |
| Using `#[test]` for async functions | Use `#[tokio::test]` instead |

# Rust Skills Checklist

This checklist is based on the [rust-skills rules](https://github.com/leonardomso/rust-skills/tree/master/rules). Use it for code reviews, onboarding, and continuous improvement.

## Linting & Formatting
- [ ] Run `cargo fmt` and `cargo clippy` with no warnings or errors.
- [ ] Enforce formatting and linting in CI.

## Documentation
- [ ] All public items are documented (`#![deny(missing_docs)]`).
- [ ] Modules and crates have top-level docs.
- [ ] Document panics, errors, and safety for unsafe code.

## Error Handling
- [ ] Use `thiserror` for custom error types.
- [ ] Avoid `unwrap`, `expect`, and panics in production code.
- [ ] Prefer `Result` and propagate errors with `?`.

## API & Type Design
- [ ] Use newtypes for IDs and domain types.
- [ ] Use enums for state and avoid stringly-typed APIs.
- [ ] Use builder patterns for complex construction.
- [ ] Mark important APIs with `#[must_use]`.

## Ownership & Memory
- [ ] Prefer borrowing over cloning.
- [ ] Use slices instead of Vec when possible.
- [ ] Use `Cow`, `Arc`, `Rc`, `Mutex` only when necessary.

## Project Structure
- [ ] Organize modules by feature, not by type.
- [ ] Use a prelude module for common imports.
- [ ] Split lib and main logic.

## Testing
- [ ] Use descriptive test names and arrange-act-assert pattern.
- [ ] Place integration tests in the `tests/` directory.
- [ ] Use `#[cfg(test)]` for unit tests.
- [ ] Avoid test interdependence and use fixtures/RAII for setup/teardown.

## Naming
- [ ] Functions: `snake_case`, Types/Enums: `CamelCase`, Constants: `SCREAMING_SNAKE_CASE`.
- [ ] Avoid `get_` prefix unless necessary.

## Async & Concurrency
- [ ] Avoid holding locks across `.await`.
- [ ] Use channels, JoinSet, and cancellation tokens appropriately.

## Performance & Optimization
- [ ] Profile before optimizing.
- [ ] Use bounds checks, cache-friendly data, and inline judiciously.

---

For the full list and explanations, see the [rust-skills rules directory](https://github.com/leonardomso/rust-skills/tree/master/rules).

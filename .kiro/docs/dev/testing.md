# Testing Guide

This guide explains how to run, write, and organize tests for this Rust monorepo project.

---

## 1. Running All Tests

To run all tests across the workspace:
```bash
cargo test
```

To run tests for a specific crate:
```bash
cargo test -p <crate-name>
```

To run a single test by name:
```bash
cargo test -p <crate-name> <test_name>
```

---

## 2. Test Organization
- Unit tests are co-located with source files in each crate, inside `#[cfg(test)] mod tests { ... }` blocks.
- There are no separate integration test directories; all tests are inline with the code.
- Both sync (`#[test]`) and async (`#[tokio::test]`) tests are used.
- Test doubles are implemented via trait impls, e.g. `impl TraitName for () { ... }`.

---


## 3. Common Test Patterns & Examples

- Use `assert_eq!`, `assert!`, and `matches!` for assertions.
- Use `tokio` for async tests.
- Use `#[should_panic]` for panic tests.
- Use `tracing` for logging inside tests.

### Example: Basic Unit Test
```rust
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add() {
		assert_eq!(2 + 2, 4);
	}
}
```

### Example: Async Test with Tokio
```rust
#[cfg(test)]
mod tests {
	use super::*;
	use tokio;

	#[tokio::test]
	async fn test_async_logic() {
		let result = async_add(2, 3).await;
		assert_eq!(result, 5);
	}

	async fn async_add(a: i32, b: i32) -> i32 {
		a + b
	}
}
```

### Example: Panic Test
```rust
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic(expected = "division by zero")]
	fn test_divide_by_zero() {
		let _ = 1 / 0;
	}
}
```

### Example: Trait-based Test Double
```rust
trait Greeter {
	fn greet(&self) -> String;
}

struct RealGreeter;
impl Greeter for RealGreeter {
	fn greet(&self) -> String {
		"Hello".to_string()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct MockGreeter;
	impl Greeter for MockGreeter {
		fn greet(&self) -> String {
			"Hi (mock)".to_string()
		}
	}

	#[test]
	fn test_greeter() {
		let greeter = MockGreeter;
		assert_eq!(greeter.greet(), "Hi (mock)");
	}
}
```

---

## 4. Linting & Formatting
- Run lints: `cargo clippy`
- Check formatting: `cargo fmt -- --check`
- Auto-format: `cargo fmt`

---

## 5. Example Commands

Run all tests for the wallet feature:
```bash
cargo test -p features-wallet-service
```

Run async tests for a core shared library:
```bash
cargo test -p shared-shared-data-core -- --test-threads=1
```

Run tests matching a pattern:
```bash
cargo test -p shared-shared-data-extractor idempotency
```

---

## 6. API Testing for Microservices

Each microservice in the `apis/` folder includes a `test.rest` file for manual or automated API testing.

### How to Run API Tests

- Open the relevant `apis/<service>/test.rest` file in VS Code.
- Use the [REST Client extension](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) to send requests directly from the editor.
- Alternatively, copy the requests to a tool like Postman, Insomnia, or use `curl`.

#### Example (VS Code REST Client):
1. Open `apis/wallet/test.rest`.
2. Click "Send Request" above any HTTP request line.
3. View the response in the editor.

#### Example (curl):
```bash
curl -X POST http://localhost:8080/api/v1/wallets -H "Content-Type: application/json" -d '{"user_id": "..."}'
```

### Notes
- Ensure the corresponding microservice is running before testing its API.
- Update or add new requests to `test.rest` as endpoints evolve.
- See the [REST Client extension docs](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) for advanced usage.

---

## 7. Notes
- See AGENTS.md for code style and test conventions.
- Add new tests in the same file as the code they test.
- Use `#[cfg(test)]` to ensure tests are not included in release builds.

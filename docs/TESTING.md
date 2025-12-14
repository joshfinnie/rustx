# Testing Guide for RustX

This document explains how to run and write tests for RustX.

## Quick Start

```bash
# Run all tests
./run-tests.sh

# Run tests in Firefox too
./run-tests.sh --firefox
```

## Test Structure

RustX has two types of tests:

### 1. Unit Tests (`src/lib.rs`)

Located in the `tests` module at the end of `src/lib.rs`. These test individual functions and components:

- Constants and markers
- DOM element creation and manipulation
- Attribute parsing
- Event creation and handling
- Form data processing
- HTTP request setup

**Count: 23 tests**

### 2. Integration Tests (`tests/integration.rs`)

Full end-to-end tests that verify the library works correctly in a browser environment:

- Element initialization with `rx-action`
- Multiple elements working together
- Form handling
- Target element selection
- `rx-ignore` functionality
- Nested elements
- All swap modes
- Custom events

**Count: 13 tests**

**Total: 36 tests**

## Running Tests

### All Tests (Recommended)

```bash
./run-tests.sh
```

This runs:
1. Unit tests in headless Chrome
2. Integration tests in headless Chrome

### Individual Test Suites

```bash
# Unit tests only
wasm-pack test --headless --chrome

# Integration tests only
wasm-pack test --headless --chrome --test integration

# Specific test by name
wasm-pack test --headless --chrome -- --test test_rustx_marker_constants
```

### Different Browsers

```bash
# Chrome (default)
wasm-pack test --headless --chrome

# Firefox
wasm-pack test --headless --firefox

# Safari (requires geckodriver)
wasm-pack test --headless --safari
```

### Interactive Browser Testing

For debugging, run tests in a real browser:

```bash
# Opens Chrome with test results
wasm-pack test --chrome

# Opens Firefox
wasm-pack test --firefox
```

## Writing Tests

### Unit Test Template

```rust
#[wasm_bindgen_test]
fn test_my_feature() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Your test code here
    let element = document.create_element("button").unwrap();
    element.set_attribute("rx-action", "/test").unwrap();

    assert_eq!(element.get_attribute("rx-action").unwrap(), "/test");
}
```

### Integration Test Template

```rust
#[wasm_bindgen_test]
fn test_integration_my_feature() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Create DOM elements
    let button = document.create_element("button").unwrap();
    button.set_attribute("rx-action", "/api/test").unwrap();
    body.append_child(&button).unwrap();

    // Verify behavior
    let found = document.query_selector("button[rx-action]").unwrap();
    assert!(found.is_some());

    // Clean up
    button.remove();
}
```

### Best Practices

1. **Always clean up DOM elements** in integration tests:
   ```rust
   // At the end of your test
   element.remove();
   ```

2. **Use descriptive test names**:
   - Good: `test_rx_action_sends_post_request`
   - Bad: `test1`

3. **Test one thing per test**:
   - Each test should verify a single behavior

4. **Use assertions effectively**:
   ```rust
   assert!(condition);
   assert_eq!(actual, expected);
   assert!(result.is_ok());
   assert!(element.is_some());
   ```

5. **Handle errors properly**:
   ```rust
   let result = something_that_might_fail();
   assert!(result.is_ok(), "Should not fail: {:?}", result.err());
   ```

## Test Categories

### DOM Manipulation Tests
- Element creation
- Attribute setting/getting
- Query selectors
- DOM traversal

### RustX Attribute Tests
- `rx-action` parsing
- `rx-method` defaults
- `rx-target` selection
- `rx-swap` modes
- `rx-trigger` events
- `rx-ignore` functionality

### Event System Tests
- Custom event creation
- Event dispatching
- Event bubbling
- Event cancellation

### Form Handling Tests
- FormData creation
- Input value extraction
- Form submission
- Query parameter building

### HTTP Request Tests
- Request initialization
- Header setting
- Method configuration
- URL building

## Continuous Integration

To run tests in CI/CD pipelines:

```yaml
# GitHub Actions example
- name: Run tests
  run: |
    cargo install wasm-pack
    wasm-pack test --headless --chrome
    wasm-pack test --headless --chrome --test integration
```

```yaml
# GitLab CI example
test:
  script:
    - cargo install wasm-pack
    - ./run-tests.sh
```

## Debugging Tests

### Enable Console Output

```rust
#[wasm_bindgen_test]
fn test_with_logging() {
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"Test started".into());

    // Your test code

    web_sys::console::log_1(&"Test finished".into());
}
```

### Run Single Test

```bash
wasm-pack test --chrome -- --test test_my_specific_test
```

### Check Test Output

```bash
# Verbose output
wasm-pack test --headless --chrome -- --nocapture

# Show all output
RUST_LOG=debug wasm-pack test --chrome
```

## Common Issues

### Tests Hang

- Make sure you're cleaning up DOM elements
- Check for infinite loops
- Verify async operations complete

### Tests Fail in CI but Pass Locally

- Ensure headless mode works: `--headless`
- Check browser versions match
- Verify WASM is built correctly

### "No tests ran"

- Make sure tests are marked with `#[wasm_bindgen_test]`
- Check `wasm_bindgen_test_configure!(run_in_browser);` is set
- Verify test file is in `tests/` or has `#[cfg(test)]`

## Performance

Current test performance:
- Unit tests: ~2-3 seconds
- Integration tests: ~3-4 seconds
- Total: ~6-7 seconds

Tips for faster tests:
1. Use `--headless` mode
2. Run tests in parallel (default)
3. Use Chrome over Firefox (faster startup)

## Code Coverage

To generate coverage (requires `cargo-tarpaulin` and `wasm-bindgen-test`):

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage (note: WASM coverage is limited)
cargo tarpaulin --out Html
```

## Adding New Tests

When adding features to RustX:

1. **Write the test first** (TDD approach):
   ```bash
   # Add test to src/lib.rs or tests/integration.rs
   # Run tests - they should fail
   ./run-tests.sh
   ```

2. **Implement the feature**

3. **Run tests - they should pass**:
   ```bash
   ./run-tests.sh
   ```

4. **Update test count** in this document and `run-tests.sh`

## Resources

- [wasm-bindgen-test documentation](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)
- [web-sys documentation](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)
- [WASM testing guide](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/browsers.html)

## Questions?

If you're adding tests and need help:
1. Look at existing tests for patterns
2. Check the documentation links above
3. Run tests in interactive mode to debug: `wasm-pack test --chrome`

---

**Happy Testing!** 🧪🦀

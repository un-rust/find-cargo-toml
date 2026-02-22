//! Minimal integration test so the template demonstrates the `tests/` layout.
//! Run with: `cargo test --test integration_test`

use package_name::hello;

#[test]
fn hello_returns_greeting() {
    assert_eq!(hello("world"), "Hello, world!");
    assert_eq!(hello("template"), "Hello, template!");
}

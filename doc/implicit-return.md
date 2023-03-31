# Implicit Return

Rust has a syntactic-sugar to implicitly `return` when a statement is missing a
semicolon; for example, these are equivalent:

```rust
fn something() -> i32 {
	return 1;
}
fn something() -> i32 {
	1
}
```

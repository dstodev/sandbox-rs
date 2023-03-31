# Ownership

- Rust types may own memory
- In C++ terms, passing by value in would instead move in Rust.
- To "borrow" data (pass by reference in C++), use the borrow operator: `&`

```rust
fn print_string(text: &str) /* borrows a str */ {
	println!("{}", text);
}

fn main() {
	let text: String = format!("{}, {}, {}", 1, 2, 3);
	print_string(&text /* borrow text */);  // prints "1, 2, 3"
}
```

This snippet "borrows" `text`, a `String` type. `String` types own memory.
However, the borrowed `text` is received by `print_string` as a `&str`: a
non-owning reference to a "slice" of a string, which may be part of or the
entirety of a string.

This means `print_string` does not own `text`, but borrows the memory owned by
`text` in order to print it.

The Rust "borrow-checker" is very strict and will cause a compilation failure if
a borrow rule is broken, but generally has useful feedback.

# Printing text to the console

- Use standard macros like `println!()`
  - These macros use a format string similar to C `printf` or Python f-strings

```rust
	fn main() {
		let a = 1;
		let b = 2;
		println!("a: {}, b: {}", a, b);  // prints "a: 1, b: 2"
	}
```

Can use `format!()` to create formatted strings without printing them

```rust
	fn main() {
		let a = 1;
		let b = 2;
		let c: String = format!("a: {}, b: {}", a, b);
		println!("{}", c);
	}
```

`c` is explicitly a `String` to show that `format!()` returns an owning data
type--the `String` type [owns its string contents.](ownership.md)

# Destructuring

Rust has ergonomic pattern-matching functionality, including one such mechanism
called "destructuring". It is easiest to describe with example:

## `match`

```rust
fn print_optional_value(value_maybe: Option<i32>) {
	match value_maybe {
		Some(value) => println!("{}", value),
		None => {}
	}
}

fn main() {
	print_optional_value(None);  // does nothing
	print_optional_value(Some(5));  // prints "5"
}
```

## `if let`

```rust
fn print_optional_value(value_maybe: Option<i32>) {
	// Destructures inner value of Option to variable, but only if Option is Some
	if let Some(value) = value_maybe {
		println!("{}", value);
	}
}

fn main() {
	print_optional_value(None);  // does nothing
	print_optional_value(Some(5));  // prints "5"
}
```

Many Rust entities are destructurable:

- [Tuples](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html)
- [Arrays & Slices](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_slice.html)
- [Enums](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_enum.html)
- [Pointers & References](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html)
- [Structs](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_structures.html)

# Struct

- Owns data
- No behavior (by default)
- Not a "class" as in other languages

## Encapsulating Data

A struct holds data, and without adding behavior, does nothing more.

```rust
struct MyData {
	an_integer: i32,
}

fn main() {
	let instance = MyData {an_integer: 2};
	println!("{}", instance.an_integer);  // prints "2"
}
```

## Providing Behavior

Structs can be provided behaviors, [*such as a constructor.*](idiomatic-struct-construction.md)
To do so, use the `impl` keyword:

```rust
struct MyData {
	an_integer: i32,
}

impl MyData {
	fn new() -> Self {
		Self {
			an_integer: 0,
		}
	}
	fn get_integer(&self) -> i32 {
		self.an_integer
	}
	fn set_integer(&mut self, new_integer: i32) {
		self.an_integer = new_integer;
	}
}

fn main() {
	let mut instance = MyData::new();
	println!("{}", instance.get_integer());  // prints "0"
	instance.set_integer(2);
	println!("{}", instance.get_integer());  // prints "2"
}
```

## Using #[derive]

Many behaviors can be provided automatically by using #[derive]. For example,
the `Clone` trait can be generated automatically for us, as long as all owned
data implements `Clone`:

```rust
#[derive(Clone)]
struct MyData {
	an_integer: i32,
}

fn main() {
	let instance = MyData {an_integer: 2};
	let cloned = instance.clone();
	println!("{}", cloned.an_integer);  // prints "2"
}
```

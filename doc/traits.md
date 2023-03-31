# Traits

- Similarities to Type Erasure pattern:
  - "Apply" behavior to types
  - Types are usable anywhere they satisfy trait constraints

- Similarities to C++20 Concepts:
  - Named constraints
  - Evaluated at compile-time

## Defining Traits

Custom traits can be defined using the `trait` keyword:

```rust
trait Printable {
	fn print(&self);
}

struct Unit;

impl Printable for Unit {
	fn print(&self) {
		println!("Unit");
	}
}

fn main() {
	let instance = Unit;
	instance.print();
}
```

This is especially useful when writing reusable code, as traits can be used as
[type bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html):

```rust
fn print_something<T: Printable>(obj: &T) {
	obj.print();
}

fn main() {
	let instance = Unit;
	print_something(&instance);
}
```

Traits are also usable with the [`impl`](https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html)
keyword if the generic parameter `T` is not important:

```rust
fn print_something(obj: &impl Printable) {
	obj.print();
}

fn main() {
	let instance = Unit;
	print_something(&instance);
}
```

and the [`dyn`](https://doc.rust-lang.org/std/keyword.dyn.html) keyword to
enable dynamic dispatch.

# Entity Visiblity

The `pub` keyword marks an entity "public"
Can be called as a function [e.g. `pub(crate)` & `pub(super)`](https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself)

Files which contain symbols not marked `pub` can still reference those symbols,
even from nested scopes:

```rust
struct Unit {
	pub a: i32,
	b: i32,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_access_both_data_members() {
		let instance = Unit { a: 0, b: 1 };
		println!("a: {}, b: {}", instance.a, instance.b);
	}
}
```

This is very commonly used to write unit tests in files which define behaviors,
as private behaviors are testable within those files:

```rust
impl Unit {
	pub fn get_a(&self) -> i32 {  // pub
		self.a
	}
	fn get_b(&self) -> i32 {  // not pub
		self.b
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_call_both_functions() {
		let instance = Unit { a: 0, b: 1 };
		println!("a: {}, b: {}", instance.get_a(), instance.get_b());
	}
}
```

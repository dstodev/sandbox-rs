struct Object {
	value: i32,
}

impl Object {
	pub fn new() -> Self {
		Self {
			value: 0,
		}
	}

	/* The must_use attribute is useful to give compiler warnings if the return value is not
	   assigned. This is a builder after all--it would not make sense to discard the return value.
	 */
	#[must_use]
	pub fn value(mut self, value: i32) -> Self {
		self.value = value;
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn construct() {
		let o = Object::new();
		assert_eq!(0, o.value);
	}

	#[test]
	fn with_value() {
		/* Variable o is not required to be mut, even though .value(1) mutates its value.
		   This is because the instance is consumed (moved-into) value() and then returned as
		   if it were a new instance.
	   */
		let o = Object::new()
			.value(1);
		assert_eq!(1, o.value);
	}

	#[test]
	fn with_value_chain() {
		let o = Object::new()
			.value(1)
			.value(2);
		assert_eq!(2, o.value);
	}

	#[test]
	fn builder_is_not_setter() {
		/* Builder methods are not setters.
		 */
		let mut o = Object::new();  // First hint: linter states o should not be marked mut
		o.value(1);
		// assert_eq!(1, o.value);  // This will not compile: Borrow of moved value: `o`
	}

	/* So, we should give a similar implementation for setters.
	   The name of the function must be different. Rust does not support that kind of function
	   overloading, but traits are usable for similar behavior.

	   This implementation is only visible in this `tests` module.
	 */
	impl Object {
		pub fn set_value(&mut self, value: i32) {
			self.value = value;
		}
	}

	#[test]
	fn builder_and_setter() {
		let mut o = Object::new()
			.value(1);
		assert_eq!(1, o.value);
		o.set_value(2);
		assert_eq!(2, o.value);
	}
}

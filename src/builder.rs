struct Object {
	value: i32,
}

impl Object {
	pub fn new() -> Self {
		Self {
			value: 0,
		}
	}
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
}

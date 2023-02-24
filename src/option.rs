struct Unit {
	value: i32,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reference_inner_value() {
		let some = Some(Unit { value: 10 });
		let mut accumulator = 0;
		let none: Option<Unit> = None;

		// Adds twice to assert the variables are not moved during the operation
		// These cannot be move-closures; captures accumulator by mutable reference
		some.as_ref().map(|unit| accumulator += unit.value);  // adds 10
		some.as_ref().map(|unit| accumulator += unit.value);  // adds 10

		none.as_ref().map(|unit| accumulator += unit.value);  // adds 0 (variable is None)

		assert_eq!(20, accumulator);
	}

	#[test]
	fn mutate_inner_value() {
		let mut some = Some(Unit { value: 10 });

		// These can be move-closures; captures nothing from environment
		some.as_mut().map(|unit| unit.value += unit.value);  // adds 10 => 20
		some.as_mut().map(|unit| unit.value += unit.value);  // adds 20 => 40

		assert_eq!(40, some.unwrap().value);
	}
}

struct Unit {
	pub a: i32,
	b: i32,
}

impl Unit {
	pub fn get_a(&self) -> i32 {
		// pub
		self.a
	}
	fn get_b(&self) -> i32 {
		// not pub
		self.b
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_access_both_data_members() {
		let instance = Unit { a: 0, b: 1 };
		println!("a: {}, b: {}", instance.a, instance.b);
	}

	#[test]
	fn can_call_both_functions() {
		let instance = Unit { a: 0, b: 1 };
		println!("a: {}, b: {}", instance.get_a(), instance.get_b());
	}
}

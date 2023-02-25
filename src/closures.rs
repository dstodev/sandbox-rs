struct Unit;

impl std::fmt::Display for Unit {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", "Unit")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn move_closure() {
		let value = Unit;
		let print_value = move || println!("{}", value); // Ownership moves on this line

		// println!("Value before move: {}", value);  // Cannot read moved variable

		print_value();
	}

	#[test]
	fn reference_closure() {
		let value = Unit;
		let print_value = || println!("{}", value); // Ownership does not move

		println!("Value before move: {}", value);

		print_value();
	}

	#[test]
	fn closure_arguments() {
		let value = Unit;
		let print_value = move |value: &Unit| println!("{}", value);
		print_value(&value);
		print_value(&value);
	}
}

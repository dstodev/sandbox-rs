fn join(collection: &[impl AsRef<str>], delimiter: &str) -> String {
	let mut joined = String::new();

	if let Some((last, up_to_last)) = collection.split_last() {
		joined = up_to_last
			.iter()
			.fold(String::new(), |acc, s| acc + s.as_ref() + delimiter);
		joined += last.as_ref();
	}

	joined
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn join_comma_space() {
		let vector = vec!["hello", "world!"];
		let joined = join(&vector, ", ");
		assert_eq!("hello, world!", joined);
	}

	#[test]
	fn join_one_element() {
		let vector = vec!["hello"];
		let joined = join(&vector, ", ");
		assert_eq!("hello", joined);
	}

	#[test]
	fn join_no_elements() {
		let vector: Vec<&str> = Vec::new();
		let joined = join(&vector, ", ");
		assert_eq!("", joined);
	}

	#[test]
	fn join_no_elements_string_class_type() {
		let vector: Vec<String> = Vec::new();
		let joined = join(&vector, ", ");
		assert_eq!("", joined);
	}
}

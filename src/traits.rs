trait Printable {
	fn print(&self);
}

struct Unit;

impl Printable for Unit {
	fn print(&self) {
		println!("Unit");
	}
}

fn print_something_1<T: Printable>(obj: &T) {
	obj.print();
}

fn print_something_2(obj: &impl Printable) {
	obj.print();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn print_instance() {
		let instance = Unit;
		instance.print();
		print_something_1(&instance);
		print_something_2(&instance);
	}
}

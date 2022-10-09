struct Object;

impl Object {
	fn new() -> Self {
		Self
	}

	fn immutable_borrow(&self) {}
	fn mutable_borrow(&mut self) {}
	fn consume(self) {}
}

/*
	Fn implements FnMut implements FnOnce

	Fn:     Callable repeatedly, can not mutate state. Captures are immutably borrowed.
	FnMut:  Callable repeatedly, can mutate state.     Captures are immutably or mutably borrowed.
	FnOnce: Callable once, can or cannot mutate state. Captures are immutably or mutably borrowed, or consumed.
 */

fn invoke_fnonce<F>(callable: F) where F: FnOnce() {
	callable();
}

fn invoke_fnmut<F>(mut callable: F) where F: FnMut() {
	callable();
}

fn invoke_fn<F>(callable: F) where F: Fn() {
	callable();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn borrow_immutably() {
		let o = Object::new();
		o.immutable_borrow();
		//o.mutable_borrow();
	}

	#[test]
	fn borrow_mutably() {
		let mut o = Object::new();
		o.immutable_borrow();
		o.mutable_borrow();
	}

	#[test]
	fn consume() {
		let o = Object::new();
		o.immutable_borrow();
		//o.mutable_borrow();
		o.consume();
		//o.consume();
	}

	#[test]
	fn consume_mutable() {
		let mut o = Object::new();
		o.immutable_borrow();
		o.mutable_borrow();
		o.consume();
		//o.consume();
	}

	#[test]
	fn capture_immutably() {
		let o = Object::new();
		let closure = || {
			o.immutable_borrow();
			//o.mutable_borrow();
		};
		closure();
		closure();
	}

	#[test]
	fn capture_mutably() {
		let mut o = Object::new();
		let mut closure = || {  // Must be mut
			o.immutable_borrow();
			o.mutable_borrow();
		};
		closure();
		closure();
	}

	#[test]
	fn capture_owning_immutably() {
		let o = Object::new();
		let closure = || {
			o.immutable_borrow();
			//o.mutable_borrow();
			o.consume();
		};
		closure();
		//closure();
	}

	#[test]
	fn capture_owning_mutably() {
		let mut o = Object::new();
		let closure = || {  // Not mut because the compiler understands it is consumed
			o.immutable_borrow();
			o.mutable_borrow();  // Even though its capture borrows mutably
			o.consume();  // Must be consumed if closure not mut
		};
		closure();
		//closure();
	}

	#[test]
	fn passing_immutable_closure() {
		let o = Object::new();
		let closure = || {  // closure implements Fn() (inferred)
			o.immutable_borrow();
		};
		invoke_fnonce(closure);
		invoke_fnmut(closure);
		invoke_fn(closure);
	}

	#[test]
	fn passing_mutable_closure() {
		let mut o = Object::new();
		let mut closure = || {  // closure implements FnMut (inferred)
			o.immutable_borrow();
			o.mutable_borrow();
		};
		invoke_fnonce(&mut closure);
		invoke_fnmut(&mut closure);
		//invoke_fn(closure);
	}

	#[test]
	fn passing_owning_closure() {
		let o = Object::new();
		let closure = || {  // closure implements FnOnce() (inferred)
			o.immutable_borrow();
			//o.mutable_borrow();
			o.consume();
		};
		invoke_fnonce(closure);
		//invoke_fnmut(closure);
		//invoke_fn(closure);
	}

	#[test]
	fn passing_owning_closure_affecting_environment() {
		let mut o = Object::new();
		let closure = || {  // closure implements FnOnce() (inferred)
			o.immutable_borrow();
			o.mutable_borrow();
			o.consume();
		};
		invoke_fnonce(closure);
		//invoke_fnmut(closure);
		//invoke_fn(closure);
	}
}

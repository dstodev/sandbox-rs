use std::sync::{Arc, Mutex, MutexGuard};

use tokio::runtime::Runtime;

struct Flag {
	value: Arc<Mutex<bool>>,
}

impl Flag {
	fn new() -> Self {
		Self {
			value: Arc::new(Mutex::new(false))
		}
	}
	fn set(&self, new_value: bool) {
		*self.get() = new_value;
	}
	fn get(&self) -> MutexGuard<bool> {
		self.value.lock().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn get_runtime() -> Runtime {
		Runtime::new().unwrap()
	}

	#[test]
	fn simple_runtime() {
		let rt = get_runtime();
		let fired = Flag::new();
		assert!(!*fired.get());
		rt.block_on(async {
			fired.set(true);
		});
		assert!(*fired.get());
	}
}

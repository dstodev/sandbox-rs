use std::sync::{Arc, Mutex, MutexGuard};

use tokio::runtime::Runtime;

struct Safe<T> {
	value: Arc<Mutex<T>>,
}

impl<T> Safe<T> {
	fn new(value: T) -> Self {
		Self {
			value: Arc::new(Mutex::new(value))
		}
	}
	fn set(&self, new_value: T) {
		*self.get() = new_value;
	}
	fn get(&self) -> MutexGuard<T> {
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
		let fired = Safe::new(false);
		assert!(!*fired.get());
		rt.block_on(async {
			fired.set(true);
		});
		assert!(*fired.get());
	}
}

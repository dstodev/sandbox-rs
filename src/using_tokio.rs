use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use tokio::runtime::Runtime;

fn get_runtime() -> Runtime {
	Runtime::new().unwrap()
}

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
		let mut lock = self.value.lock().unwrap();
		let value = lock.deref_mut();
		*value = new_value;
	}
	fn get(&self) -> bool {
		let lock = self.value.lock().unwrap();
		let value = lock.deref();
		*value
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simple_runtime() {
		let rt = get_runtime();
		let fired = Flag::new();
		assert!(!fired.get());
		rt.block_on(async {
			fired.set(true);
		});
		assert!(fired.get());
	}
}

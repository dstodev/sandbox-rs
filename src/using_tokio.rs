use std::sync::{Arc, Mutex, MutexGuard};

use tokio::runtime::Runtime;

//#[derive(Clone)]  derived version implements clone() returning &Self instead of Self (?)
struct Safe<T> {
	value: Arc<Mutex<T>>,
}

impl<T> Clone for Safe<T> {
	fn clone(&self) -> Self {
		Self {
			value: self.value.clone(),
		}
	}
}

impl<T> Safe<T> {
	/// Wraps `value` in a ref-counted thread-safe mutex. As long as the returned instance
	/// or a handle() of this instance is alive, the owned instance is alive.
	fn new(value: T) -> Self {
		Self {
			value: Arc::new(Mutex::new(value))
		}
	}
	fn lock(&self) -> MutexGuard<T> {
		self.value.lock().unwrap()
	}
	fn handle(&self) -> Self {
		self.clone()
	}
}

impl<T> Safe<T> where T: Clone {
	fn view(&self) -> T {
		self.lock().clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simple_runtime() {
		let rt = Runtime::new().unwrap();
		let fired = Safe::new(false);

		assert!(!fired.view());

		rt.block_on(async {
			*fired.lock() = true;
		});

		assert!(fired.view());
	}

	#[test]
	fn simple_runtime_move() {
		let rt = Runtime::new().unwrap();
		let fired = Safe::new(false);
		let handle = fired.handle();

		rt.block_on(async move {
			*handle.lock() = true;
		});

		assert!(fired.view());
	}

	#[test]
	fn multiple_workers() {
		let rt = Runtime::new().unwrap();
		let counter = Safe::new(0);

		let start_adder_task = |incr| {
			let handle = counter.handle();
			rt.spawn(async move {
				/* The mutex `lock` is locked for the duration of the task to avoid TOCTOU errors.
				     Once locked, the lock can be used where the underlying type can,
				     except sometimes explicitly dereferenced like `*lock`.

				     In test output, note that the order by which numbers are added
				     is random but the counter maintains continuity between each add:

				     1 + 0 = 1  // order: This set adds in-order: +1 then +2 then +3
				     2 + 1 = 3  // continuity: The right-hand operand of each row
				     3 + 3 = 6                 is the result of the previous row
				     -----
				     1 + 6 = 7    // order: This set adds out-of-order: +1 then +3 then +2
				     3 + 7 = 10   // continuity: The right-hand operand of each row
				     2 + 10 = 12                 is the result of the previous row
				*/
				let mut lock = handle.lock();
				println!("{} + {} = {}", incr, *lock, incr + *lock);
				*lock += incr;
			})  // returns a JoinHandle
		};

		let iterations = 10;
		let sum_of_adders = 6;

		/* [x..y), [x..=y] */
		for i in 1..=iterations {
			println!();

			let running_tasks = vec![
				start_adder_task(1),  // This task will usually lock the mutex first
				start_adder_task(2),
				start_adder_task(3),
				// Sum of adders is 6
			];

			for task in &running_tasks {
				while !task.is_finished() { /* Tasks are running. Wait here... */ }
			}

			assert_eq!(i * sum_of_adders, counter.view());
		}
		assert_eq!(iterations * sum_of_adders, counter.view());
	}

	#[tokio::test]
	async fn multiple_workers_tokio() {
		// Order is preserved
		multiple_workers_tokio_body().await;
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn multiple_workers_tokio_multithreaded() {
		// Order is not preserved
		multiple_workers_tokio_body().await;
	}

	async fn multiple_workers_tokio_body() {
		let counter = Safe::new(0);

		let start_adder_task = |incr| {
			let handle = counter.handle();
			tokio::spawn(async move {
				let mut lock = handle.lock();
				println!("{} + {} = {}", incr, *lock, incr + *lock);
				*lock += incr;
			})
		};

		let iterations = 10;
		let sum_of_adders = 6;

		for i in 1..=iterations {
			println!();

			let _ = tokio::join!(
				start_adder_task(1),
				start_adder_task(2),
				start_adder_task(3),
			);

			assert_eq!(i * sum_of_adders, counter.view());
		}
		assert_eq!(iterations * sum_of_adders, counter.view());
	}
}

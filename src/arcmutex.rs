use std::sync::{Arc, Mutex};

struct Object {
	value: i32,
}

impl Object {
	fn new() -> Self {
		Self {
			value: 0,
		}
	}
}

/* Arc<Mutex<T>> gives type T "Interior Mutability" with additional thread-safety;

   "Arc" stands for "Atomically Reference Counted"
   Related type Rc<T> exists which is not atomic and not thread-safe.
   However, reference-counting atomically is more computationally expensive.
 */
type ArcMutex<T> = Arc<Mutex<T>>;

type MagicObject = ArcMutex<Object>;

/* Implementing this trait lets us construct a MagicObject like:

     let o = MagicObject::from(Object::new());
       or
     let o: MagicObject = Object::new().into();

   trait Into<U> for T is implemented automatically by impl From<T> for U
 */
impl From<Object> for MagicObject {
	fn from(o: Object) -> Self {
		Arc::new(Mutex::new(o))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn construct_object() {
		let _ = Object::new();
	}

	#[test]
	fn construct_magicobject() {
		let _ = MagicObject::from(Object::new());
		let _: MagicObject = Object::new().into();
	}

	#[test]
	fn object_cannot_mutate_if_not_marked_mut() {
		let o = Object::new();  // The object is immutable
		//let value = &mut o.value;  // "Cannot borrow immutable local variable `o.value` as mutable"
		assert_eq!(0, o.value);
	}

	#[test]
	fn magicobject_can_mutate_if_not_marked_mut() {
		let o: MagicObject = Object::new().into();  // The object is immutable
		let value = &mut o.lock().unwrap().value;  // Its interior is mutable
		*value = 1;
		assert_eq!(1, *value);
	}

	#[test]
	fn magicobject_other_styles() {
		let o: MagicObject = Object::new().into();
		match o.lock() {
			Ok(mut interior) => {
				interior.value = 1;
				assert_eq!(1, interior.value);
			}
			Err(guard) => panic!("Mutex was poisoned!"),
		};
	}
}

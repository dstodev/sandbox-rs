# Idiomatic Struct Construction

Types in rust typically define a constructor function called `new()`:

```rust
struct MyData {
  an_integer: i32,
}

impl MyData {
  pub fn new() -> Self {
    Self {
      an_integer: 0
    }
  }
}

fn main() {
	let instance = MyData::new();
	println!("{}", instance.an_integer);
}
```

`new()` returns `Self`; `Self` is shorthand to mean "the current class" i.e.
`MyData` in this case. The function body of `new` is simply an inline
instantiation of the type, which is [implicitly returned.](implicit-return.md)

So, the above constructor is equivalent to:

```rust
impl MyData {
  pub fn new() -> MyData {
    let new_instance = MyData {
      an_integer: 0
    };
    return new_instance;
  }
}
```

To pass parameters to the constructor, simply add arguments to `new()`:

```rust
impl MyData {
  pub fn new(an_integer: i32) -> Self {
    Self {
      an_integer: an_integer
    }
  }
}
```

An additonal syntactic-sugar is for parameter names which bear the same name as
the data member to initialize, for example `an_integer` (the data member) and
`an_integer` (the function parameter). When this is the case, the constructor is
even further reduceable:

```rust
impl MyData {
  pub fn new(an_integer: i32) -> Self {
    Self {
      an_integer
    }
  }
}
```

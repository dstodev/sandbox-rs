# Mutability

Rust variables are constant by default. This means the following code does not
compile:

```rust
fn main() {
  let a = 1;
  a = 2;  // Does not compile; a is immutable
}
```

To mark a variable as mutable, simply use the `mut` keyword:

```rust
fn main() {
  let mut a = 1;
  a = 2;  // Compiles; a is mutable
}
```

Generally, the Rust compiler is good at telling you when a variable needs to be
marked `mut`, but consider what it means to do so: perhaps the variable does not
need to be mutable. For example, you could instead re-use the same name, but
assign to a new variable:

```rust
fn main() {
  let a = 1;
  let a = 2;  // Compiles; a is overwritten
}
```

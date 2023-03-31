# Closures

Rust functions are [first-class citizens](https://en.wikipedia.org/wiki/First-class_citizen),
meaning that they are treated like any other type instance in Rust. You could
store one in a variable, for example:

```rust
fn get_one() -> i32 {
	1
}

fn main() {
	let one_fn = get_one;
	let one = one_fn();
	println!("{}", one);  // prints "1"
}
```

However, passing static functions around is typically uncommon. A more ergonomic
way to express this may be with a Closure.

## What Is a Closure?

A closure is an inline function similar to a lambda expression in languages like
C++. There are two types of closures: capture-by-reference & capture-by-move.

### Capture by Move

Syntax for a move-closure looks like:

```rust
let print_value = move || println!("{}", value);  // Ownership moves on this line
```

I typically use this type of closure first, as it is the most-strict to write,
but least-strict to use. If usage restricts it, though, then you might consider
a reference-closure.

### Capture by Reference

Syntax for a reference-closure looks like:

```rust
let print_value = || println!("{}", value);  // Ownership does not move
```

## Closure Arguments

Closures can take arguments like so:

```rust
let print_value = move |value: &Unit| println!("{}", value);
```

Note that even though this is a move-closure, the argument `value` is borrowed.
In this example, `move` is superfluous (nothing is captured), but I still
recommend the keyword as a good rule-of-thumb.

## Damage to Control Flow Interpretability

As in all languages with similar functionality, one should take care to minimize
the scope of closures, as they quickly increase control flow complexity. Try not
try return closures from functions, for example, and restrict their declaration
& usage to specific scopes.

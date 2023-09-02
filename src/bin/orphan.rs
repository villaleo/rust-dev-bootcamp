// ---
// Orphan Rule:
// In order to implement a trait on a given type, either the trait or the type must be defined
// within the current crate.
//
// Without this rule, two crates could implement the same traits for a type and Rust wouldn't be
// able to tell which implementation to use.
//
// There is, however a workout. We can wrap a type in a new type using a tuple struct, and
// implement the trait for the wrapper.
// --

fn main() {}

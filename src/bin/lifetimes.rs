#![allow(dead_code)]

// There are concrete and generic lifetimes.
// Non-lexical lifetime is when value's lifetimes are not tied to scope. Lifetimes help the borrow
// checker catch and prevent dangling references.
// A returned reference must live at least as long as the shortest lifetime specified in the parameters.

// Example of concrete, non-lexical lifetimes
fn non_lexical_lt_ex() {
    let mut s1 = String::from("foo");
    let r1 = &s1;
    println!("r1: {r1}"); // Since rust knows this is the last time r1 is used...
    let r2 = &mut s1; // we are allowed to create a &mut
    r2.push_str("!");
}

// Generic lifetimes specify the relationship between lifetimes of references. In general, the
// lifetime of the return value will be the same as one of the input parameters
// Example of generic lifetimes
fn generic_lt_ex<'a>(player_a: &'a str, player_b: &'a str) -> &'a str {
    // player_a, player_b, and return value have the same lifetime, specified by 'a
    let some_condition = true;

    if some_condition {
        player_a
    } else {
        player_b
    }
}

// Static lifetimes are references that live throughout the entire app's life. These are commonly
// used when working with string slices.
fn static_lt_ex() -> &'static str {
    let s: &'static str = "💕";
    s
}

fn main() {}

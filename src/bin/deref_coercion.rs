#![allow(dead_code)]

// Deref coercion allos the compiler to coerce a reference of one type to a reference of
// another type.
// This happens implicityly when references are passed to functions or methods.
// Deref coercion only works on types that implement the Deref and DerefMut traits.
// These traits should only be implemented for smart pointer types.

use std::ops::{Deref, DerefMut};

// pub trait std::ops::Deref {
//     type Target: ?Sized;
//     fn deref(&self) -> &Self::Target;
// }
//
// pub trait std::ops::DerefMut {
//     fn deref_mut(&mut self) -> &mut Self::Target;
// }
//

struct MySmartPtr<T> {
    val: T,
}

impl<T> MySmartPtr<T> {
    fn new(val: T) -> Self {
        Self { val }
    }
}

impl<T> Deref for MySmartPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> DerefMut for MySmartPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

fn main() {
    let s = MySmartPtr::new(Box::new("Rusty".to_owned()));
    // In this case:
    // &MySmartPtr -> &Box -> &String -> &str
    print_str(&s);

    // Additional derefs lead to this cursed abomonation.
    print_str(&(***s));
}

fn print_str(s: &str) {
    println!("{}", s);
}

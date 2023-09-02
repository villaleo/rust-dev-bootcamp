#![allow(dead_code)]

use std::rc::Rc;
// Rc can only be used in single-threaded applications. For multi-threaded applications, you must
// use Arc.

struct Database {}

struct AuthService {
    db: Rc<Database>,
}

struct ContentService {
    db: Rc<Database>,
}

fn main() {
    let db = Rc::new(Database {});

    // Rc::clone does not clone the instance, but it instead increments the reference count.
    let _auth_service = AuthService { db: Rc::clone(&db) };
    let _content_service = ContentService { db: Rc::clone(&db) };

    // We can get the number of references behind an Rc value using Rc::strong_count
    assert_eq!(Rc::strong_count(&db), 3);
}

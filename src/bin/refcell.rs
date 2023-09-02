#![allow(dead_code)]

// Rc only allows immutable shared ownership of a value.
use std::rc::Rc;

// We can use RefCell to get around this. RefCell uses the interior mutability design pattern. This
// pattern allows us to mutably borrow data even when there's an immutable reference to that data.
//
// Use RefCell with caution, as the responsibility of following the borrowing rules is on the
// developer.
use std::cell::RefCell;

struct Database {
    max_cons: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database { max_cons: 100 }));

    let _auth_service = AuthService { db: Rc::clone(&db) };
    let _content_service = ContentService { db: Rc::clone(&db) };

    db.borrow_mut().max_cons = 200;
}

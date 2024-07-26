enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::{rc::Rc, thread::spawn};
use List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("1) count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("2) count after creating a = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("3) count after creating a = {}", Rc::strong_count(&a));
    }

    println!(
        "4) count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );


    // interior mutability: Mutable borrow to immutable value.
}

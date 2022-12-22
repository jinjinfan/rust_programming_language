use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons,Nil};

#[derive(Debug)]
enum RefCycleList {
    Cons(i32, RefCell<Rc<RefCycleList>>),
    Nil,
}

impl RefCycleList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCycleList>>> {
        match self {
            RefCycleList::Cons(_, item) => Some(item),
            RefCycleList::Nil => None,
        }
    }
}

#[derive(Debug)]
enum MutableList {
    Cons(Rc<RefCell<i32>>, Rc<MutableList>),
    Nil,
}


// Define own shared pointer
struct MyBox<T:std::fmt::Display>(T);

impl<T:std::fmt::Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T:std::fmt::Display> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T:std::fmt::Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping Mybox smart pointer with data `{}`!", self.0);
    }
}
fn main() {
    // Reference count
    /*let a = Rc::new(Cons(2, 
                Rc::new(Cons(3, 
                    Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(4,Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let c = Cons(5,Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
    }
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Own Box type           
    let x = 5;
    let y = MyBox::new(x);
    let z = MyBox::new(8);
    drop(y);
    println!("ence smart pointer: {}", *z);

    // Interior Mutability Pattern
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(MutableList::Cons(Rc::clone(&value), Rc::new(MutableList::Nil)));
    let b = MutableList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = MutableList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    */
    // Reference cycle
    let a = Rc::new(RefCycleList::Cons(5, RefCell::new(Rc::new(RefCycleList::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(RefCycleList::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation ={}", Rc::strong_count(&a));
    println!("b inital rc count ={}", Rc::strong_count(&b));
    println!("b next item ={:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}

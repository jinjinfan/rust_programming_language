fn unsafe_deref_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }

}

use std::slice;

// Create safe abstraction layer over unsafe code
fn split_at_mut(slice : &mut[i32], mid:usize)->(&mut[i32], &mut[i32]) {
    let len = slice.len();
    assert!(mid <= len);
    let ptr = slice.as_mut_ptr();
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len-mid))
    }

}

fn call_unsafe_funs() {
    let mut vec = vec![1,2,3,4,5,6];
    let (a, b) = split_at_mut(&mut vec, 3);
    println!("First part : {:?}", a);
    println!("Second part : {:?}", b);
}

extern "C" {
    fn abs(input:i32) -> i32;
}

fn extern_c()
{
    unsafe{
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from c!");
}

static mut COUNTER:u32 = 0;
fn modify_mutable_static_variable(inc:u32) {
    unsafe {
        COUNTER += inc;
        println!("COUNTER: {}", COUNTER);
    }
}

// unsafe trait
unsafe trait Foo {
    
}

unsafe impl Foo for i32 {

}

//Operator overload
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x : i32,
    y : i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}

fn overload_operator() {
    struct Millemeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millemeters {
        type Output = Millemeters;
        fn add(self, other:Meters) -> Millemeters {
            Millemeters(self.0 + (other.0) * 1000)
        }
    }
    let Point{x, y} = Point{x:1, y:0} + Point{x:2, y:3};
    println!("Point x = {}", x);
    println!("Point y = {}", y);

}

//Fully qualified syntax for disambiguation
fn fully_qualified_syntax(){
    trait Pilot {
        fn fly(&self);
        fn name()-> String;
    }
    
    trait Wizard {
        fn fly(&self);
    }
    
    struct Human {}
    
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking!")
        }
        fn name() -> String {
            String::from("Pilot human")
        }
    }
    
    impl Wizard for Human {
        fn fly(&self) {
            println!("UP!")
        }
    }
    
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*")
        }
        fn name() -> String {
            String::from("Human")
        }
    }
    let person = Human{};
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    println!("A person name: {}", Human::name());
    println!("A person name: {}", <Human as Pilot>::name());
}

fn supertrait() {
    use std::fmt;
    trait OutlinePrint:fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len+4));
            println!("*{}*", "*".repeat(len+2));
            println!("* {} *", output);
            println!("*{}*", "*".repeat(len+2));
            println!("{}", "*".repeat(len+4));
        }
    }

    impl OutlinePrint for Point {}
    impl fmt::Display for Point {
        fn fmt(&self, f :&mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    let p = Point{x:3,y:4};
    p.outline_print();
}


fn newtype_pattern() {
    use std::fmt;
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }
    let w = Wrapper(vec![String::from("hello"),String::from("world") ]);
    println!("w = {}", w);
}

fn dynamic_sized_type_trait () {
    fn generic<T:?Sized>(t:&T) { // T may or may not be Sized

    }
}

fn function_pointer() {
    fn add_one(x:i32) -> i32 {
        x+1
    }
    fn do_twice(f: fn(i32)-> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1,2,3];
    let list_of_strings : Vec<String> = list_of_numbers
                .iter()
                .map(|i|i.to_string())
                .collect();
    println!("List of strings: {:?}", list_of_strings);
    let list_of_strings2 : Vec<String> = list_of_numbers
                .iter()
                .map(ToString::to_string)
                .collect();
    println!("List of strings: {:?}", list_of_strings2);   
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses : Vec<Status> = 
        (0u32..20)
        .map(Status::Value)
        .collect();
    println!("List of list_of_statuses: {:?}", list_of_statuses);   
    
    fn return_closure() -> Box<dyn Fn(i32)->i32> {
        Box::new(|x|x+1)
    }
}

fn main() {
    unsafe_deref_raw_pointer();
    call_unsafe_funs();
    extern_c();
    modify_mutable_static_variable(21);
    overload_operator();
    fully_qualified_syntax();
    supertrait();
    newtype_pattern();
    dynamic_sized_type_trait();
    function_pointer();
}

use std::thread;
use std::time::Duration;


fn generate_workout(intensity:u32, random_number:u32) {
    let mut expensive_closure = Cacher::new(|num:u32| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }

}

// Use generic and trait bound to define a structure to hold closure and a return value
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    caculation : T,
    value      : Option<u32>,
}

impl<T> Cacher<T>
    where T : Fn(u32) -> u32
{
    fn new(caculation : T) -> Cacher<T> {
        Cacher {
            caculation,
            value : None
        }
    }

    fn value(&mut self, arg : u32) -> u32 {
        match self.value {
            Some(v) => v,
            None    => {
                let v = (self.caculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    generate_workout(10, 3);
    //let add_one = |x:u32| x+1;
    //println!("1 + 1 = {}", add_one(1));

    // Move enviroment variable
    let x = vec![1, 2, 3];
    //let equal_to_x = move |z:Vec<u32>| z == x; // Similar to FnOnce
    let equal_to_x = |z:Vec<u32>| z == x;
    println!("{:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

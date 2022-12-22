use std::collections::HashMap;

#[derive(Debug)]
struct CarPool {
    passengers : Vec<String>,
}

impl CarPool {
    fn pick_up(&mut self, name : String) {
        self.passengers.push(name);
    }
}

fn main() {
    let s = String::from("book");
    println!(
        "I have one {}, you have two {}",
        s,
        pluralize(&s),
    );

    //
    let a = [1, 2, 3];
    let v = vec![4, 5, 6];
    let v_slice = &v[..];
    only_reference_to_array(&a);
    only_reference_to_vector(&v);
    only_reference_to_either_array_or_vector(&a);
    only_reference_to_either_array_or_vector(&v);
    only_reference_to_either_array_or_vector(&v_slice[..1]);

    //
    let mut monday_car_pool = CarPool {
        passengers : vec![],
    };
    monday_car_pool.pick_up(String::from("Jack"));
    println!("Carpool state: {:?}", monday_car_pool);
    monday_car_pool.pick_up(String::from("Lucy"));
    println!("Carpool state: {:?}", monday_car_pool);

    //
    let text = "hello world hello";
    let mut freqs = HashMap::new();
    for word in text.split_whitespace() {
        *freqs.entry(word).or_insert(0) += 1;
    }
    println!("Word frequencies: {:#?}", freqs);

}

fn only_reference_to_array(param : &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

fn only_reference_to_vector(param : &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

fn only_reference_to_either_array_or_vector(param : &[i32]) {
    println!("this is a slice: {:?}", param);
}

fn pluralize(singular : &str) -> String {
    singular.to_owned() + "s"
}
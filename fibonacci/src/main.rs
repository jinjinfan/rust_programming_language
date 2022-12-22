use std::io;

fn fibo(num : u32) -> u32 {
    let res = match num {
        0|1 => 1,
        _ => fibo(num-1) + fibo(num - 2),
    };
    res
}

fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

fn main() {
    /*println!("Please input your guess.");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num :u32 = match num.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("Please enter a valid numver"),
    };
    println!("You entered: {}", num);
    println!("Result : {}", fibo(num));
    */
    let whole_string = String::from("hello world");
    let word = first_word(&whole_string);
    println!("First word is {}", word);
    let string_slice = "hello world";
    println!("First word is {}", first_word(string_slice));

}

fn iflet() {
    let favorite_color : Option<&str> = None;
    let is_tuesday = false;
    let age : Result<u8,_> = "34".parse();

    if let Some(color) = favorite_color{
        println!("Using your favorite color, {}, as the backgroud", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn whilelet() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("while let {}", top);
    }
}

fn forloop() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("for {} is at index {}", value, index);
    }
}

enum Message {
    Hello{id: i32},
}

fn binding() {
    let msg = Message::Hello{id:5};
    match msg {
        Message::Hello {id:id_variable @ 3..=7} => {
            println!("Find an id in range: {}", id_variable);
        }
        Message::Hello{id:10..=12} => {
            println!("Find an is in range 10 - 12");
        }
        Message::Hello{id} => {
            println!("Find some other id: {}", id);
        }
    }
}

fn main() {
    iflet();
    whilelet();
    forloop();
    binding();
}

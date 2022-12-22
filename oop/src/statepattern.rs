use oop::{Screen, SelectBox, Button, Post};

fn main() {
    let screen = Screen{
        components: vec![
            Box::new(SelectBox {
                width : 75,
                height : 10,
                options : vec![String::from("yes"),String::from("no")],
            }),
            Box::new(Button {
                width : 75,
                height : 10,
                label : String::from("OK"),
            }),
        ]
    };
    screen.run();

    // State pattern
    // Blog post
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

}

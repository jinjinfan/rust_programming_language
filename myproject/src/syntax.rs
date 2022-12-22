use std::io;

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

enum HockeyPostion {
    Wing,
    Center,
    Defense,
    Goatie,
}

struct HockeyPlayer {
    name : String,
    number : u8,
    position : HockeyPostion,
    goal_ytd : u8,
}

fn tell_time(clock : Clock) {
    match clock {
        Clock::Sundial(hours) => println!("It is about {} o'clock", hours),
        Clock::Analog(hours, minutes, seconds) => {
            println!(
                "It is {} minutes and {} seconds pass {} o'clock",
                minutes, seconds, hours,
            );
        },
        Clock::Digital(hours, minutes) =>
            println!("It is {} minutes past {} o'clock", minutes, hours),
    }
}

// tuple struct
struct Triangle(u32, u32, u32);
// unit struct
struct MyStruct;

fn main() {
    // Discout
    let amount = discount(50);
    println!("Discount is {}", amount);
    /*loop {
        println!("What is the secret word?");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        if word.trim() == "rust" {
            break;
        }
    }*/
    /*for i in 1..11 {
        println!("Now serving number {}", i);
    }*/
    /*let die1 = 1;
    let die2 = 5;
    match (die1, die2) {
        (1, 1) => println!("snake die!"),
        (5, _) | (_, 5) => {
            println!("at least one 5");
            println!("Move and then roll again!");
        },
        _ => println!("Move your piece!"),
    }*/
    tell_time(Clock::Analog(9,25,45));
    let triangle1 = Triangle(3,4,5);
    is_equilateral(triangle1);

    // Hockeyplayer
    let mut player = HockeyPlayer::new(
        String::from("Bryan Rust"),
        17,
        HockeyPostion::Wing,
    );
    player.shoot_puck(1000);

}

// associate functions to create instance of type

impl HockeyPlayer {
    fn new (name : String, number: u8, position: HockeyPostion) -> HockeyPlayer {
        HockeyPlayer {
            name : name,
            number : number,
            position : position,
            goal_ytd : 0,
        }
    }
    fn shoot_puck (self, seconds_remaining : u16) {
        if seconds_remaining < 300 {
            match self.position {
                HockeyPostion::Center => println!("Goal!"),
                _ => println!("Miss!"),
            }
        } else {
            println!("Goal!");
        }
    }
}

fn is_equilateral(triangle : Triangle) -> bool {
    triangle.0 == triangle.1 && triangle.1 == triangle.2
}

fn discount(day_of_month : u8) -> i32{
    let amount = if day_of_month % 2 == 0 {
        50
    } else {
        10
    };
    amount
}

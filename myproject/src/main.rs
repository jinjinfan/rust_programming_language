extern crate rand;

fn simulate_game<'a>(home : &'a str, away : &'a str) -> &'a str {
	if rand::random() {
		home
	} else {
		away
	}
}

pub struct Stemmer {
	pub suffix : String,
}

impl Stemmer {
	pub fn stem<'a>(&self, word : &'a str) -> &'a str {
		if word.ends_with(&self.suffix) {
			let index = word
				.rfind(&self.suffix)
				.expect("Should be found because ends_with returned true");
			&word[0..index]
		} else {
			word
		}
	}
}
fn main() {
	let team1 = String::from("Panthers");
	{
		let team2 = String::from("Yellow Jackets");
		let winner = simulate_game(&team1, &team2);
		println!("{} VS. {}: {} won", team1, team2, winner);
	}
	println!("Can still discuss the {} here", team1);

	let word = String::from("credited");
	let word_stem = {
		let stemmer = Stemmer {
			suffix : String::from("ed"),
		};
		stemmer.stem(&word)
	};
	println!("The stem of {} is {}", word, word_stem);
}
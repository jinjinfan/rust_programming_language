use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::fmt;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person {
	name : String
}

impl Default for Person {
	fn default() -> Self {
		Person {
			name : String::from("unknown"),
		}
	}
}
fn main() {
	//
	let first = serde_json::from_str::<Person> (r#"{
		"name" : "Marge Hale"
	}"#);
	/*let first_inner = match first {
		Ok(inner) => inner,
		Err(error) => panic!("could not parse JSON into Person!"),
		};
	println!("first's name = {:?}", first_inner.name);*/
	//let first_inner = first.unwrap_or(Person {name : String::from("unknown")});
	let first_inner = first.unwrap_or_default();
	println!("first's name = {:?}", first_inner.name);
	//Custom error type
	/*if let Err(e) = run_application() {
		panic!("An error happened: {}", e);
	}*/

}

// Document storage service
const MAX_DOCS_CREATED_PER_MINUTE : u8 = 100;

fn num_documents_created_in_last_minute() ->  u8 {
	2
}

#[derive(Debug)]
pub enum DocumentServiceError {
	RateLimitExceed,
	Io(io::Error),
}

impl Error for DocumentServiceError {
	fn description(&self) -> &str {
		use DocumentServiceError::*;
		match *self {
			RateLimitExceed => "rate limit exceeded",
			Io(_) => "I/O error",
		}
	}
}

impl fmt::Display for DocumentServiceError {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		use DocumentServiceError::*;
		match *self {
			RateLimitExceed => write!(
				f,
				"You have exceeded the allowed number of documents per minute."
			),
			Io(ref io) => write!(f, "I/O error: {}", io),
		}
	}
}

impl From<io::Error> for DocumentServiceError {
	fn from(other : io::Error) -> Self {
		DocumentServiceError::Io(other)
	}
}

use std::result;

pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn create_document2(filename: &str) -> Result<File> {
	if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
		return Err(DocumentServiceError::RateLimitExceed)
	}
	let file = OpenOptions::new()
						.write(true)
						.create_new(true)
						.open(filename)?;
	Ok(file)
}

fn create_project(project_name : &str) -> Result<()> {
	create_document2(&format!("{}-draft1", project_name))?;
	create_document2(&format!("{}-draft2", project_name))?;
	create_document2(&format!("{}-revision1", project_name))?;
	create_document2(&format!("{}-revision2", project_name))?;
	Ok(())
}
/*
// Multiple error types
fn num_threads() -> Result<usize, Box<dyn Error>> {
	let s = env::var("NUM_THREADS")?;
	let n: usize = s.parse()?;
	Ok(n+1)
}

fn run_application() -> Result <(), Box<dyn Error>> {
	let num = num_threads()?;
	println!("the number of threads is {}", num);
	Ok(())
}

// use question mark
fn save_status(text : &str) -> Result<i64, &'static str> {
	if text.len() > 200 {
		return Err("status text is too long");
	} else {
		let record = status_to_database(text)?;
		Ok(record.id)
	}
}

fn status_to_database(text : &str) -> Result<StatusRecord, &'static str> {
	Err("database unavailable")
}

struct StatusRecord {
	id : i64,
	text : String,
	created_at : std::time::Instant,
}*/
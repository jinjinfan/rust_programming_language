#[macro_use]
extern crate quick_error;

use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use quick_error::ResultExt;

fn main() {
	match create_project("my-project") {
		Ok(()) => println!("Project created successfully"),
		Err(e) => println!("Project created failed: {}", e),
	}

}

// Document storage service
const MAX_DOCS_CREATED_PER_MINUTE : u8 = 100;

fn num_documents_created_in_last_minute() ->  u8 {
	2
}

quick_error! {
	#[derive(Debug)]
	pub enum DocumentServiceError {
		RateLimitExceed {
			display("You have exceed the allowed number of documents per minute.")
		}
		Io(filename: String, cause : io::Error) {
			display("I/O error, {} for filename {}", cause, filename)
			context(filename:&'a str, cause: io::Error)
				-> (filename.to_string(), cause)
		}
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
						.open(filename)
						.context(filename)?;
	Ok(file)
}

fn create_project(project_name : &str) -> Result<()> {
	create_document2(&format!("{}-draft1", project_name))?;
	create_document2(&format!("{}-draft2", project_name))?;
	create_document2(&format!("{}-revision1", project_name))?;
	create_document2(&format!("{}-revision2", project_name))?;
	Ok(())
}

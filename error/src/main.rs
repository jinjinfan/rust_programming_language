use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::io::Read;
use std::fs;
fn read_username_from_file() -> Result<String, std::io::Error> {
    //let mut f = File::open("hello.txt")?;
    /*let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)*/
    fs::read_to_string("hello1.txt")
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let f = File::create("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };
    f.write("Hello world".as_bytes()).expect("Write failed");

    let f = File::open("Hello1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello1.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file:{:?}", error);
            })
        } else {
            panic!("Problem opening the file:{:?}", error);
        }
    });

    let f = File::open("Hello.txt")?;
    Ok(())
}

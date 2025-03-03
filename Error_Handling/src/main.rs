use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?; // `?` returns error if File::open() fails
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    // let file_result = File::open("hello.txt");

    // let file = match file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => {
    //             println!("File not found, creating a new one...");
    //             File::create("hello.txt").unwrap()
    //             // File::create("hello.txt").unwrap()
    //         }
    //         other_error => {
    //             panic!("Problem opening file: {:?}", other_error);
    //         }
    //     },
    // };

    // panic!("Something went wrong!");

    // Using unwrap() and expect()

    // let file = File::open("hello.txt").unwrap();
    // let file = File::open("hello.txt").expect("Failed to open file");

    match read_file() {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {:?}", error),
    }
}

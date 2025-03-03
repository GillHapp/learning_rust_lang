use std::fs::File;
use std::io::ErrorKind;

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

    let file = File::open("hello.txt").unwrap();
    let file = File::open("hello.txt").expect("Failed to open file");
}

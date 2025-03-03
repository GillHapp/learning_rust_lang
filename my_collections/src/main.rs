// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }
// hashmap

use std::collections::HashMap;
fn main() {
    // println!("Hello, world!");
    // let mut numbers: Vec<i32> = Vec::new(); // Create an empty vector
    // numbers.push(1);
    // numbers.push(2);
    // numbers.push(3);
    // println!("{:?}", numbers); // Output: [1, 2, 3]

    // // Create a vector with elements using macros

    // let mut numbers = vec![10, 20, 30, 40];
    // println!("{:?}", numbers); // Output: [10, 20, 30, 40]
    // println!("{}", numbers[1]); // Output: 10

    // match numbers.get(2) {
    //     Some(value) => println!("The value is {}", value),
    //     None => println!("No value found"),
    // }

    // for num in &numbers {
    //     println!("{}", num);
    // }

    // let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    // println!("{:?}", doubled); // Output: [20, 40, 60, 80]

    // numbers.push(4); // Add 4 at the end
    // println!("{:?}", numbers); // Output: [1, 2, 3, 4]

    // numbers.pop(); // Removes last element
    // println!("{:?}", numbers); // Output: [1, 2, 3]

    // // let first = &numbers[0]; // Immutable borrow
    // // println!("{}", first);
    // // numbers.push(4); //
    // // println!("{:?}", numbers); // Output:

    // // alternate

    // let mut numbers = vec![1, 2, 3];

    // if let Some(first) = numbers.get(0) {
    //     println!("First element: {}", first);
    // }

    // numbers.push(4); // ✅ No issue

    // let people = vec![
    //     Person {
    //         name: "Alice".to_string(),
    //         age: 25,
    //     },
    //     Person {
    //         name: "Bob".to_string(),
    //         age: 30,
    //     },
    // ];

    // for person in &people {
    //     println!("{:?}", person);
    // }

    // strings in rust

    // let mut s = String::new(); // Creates an empty string
    // s.push_str("Hello, Rust!"); // Adding text
    // println!("{}", s); // Output: Hello, Rust!

    // let s = "Hello".to_string();
    // println!("{}", s); // Output: Hello

    // let s = String::from("Rust is awesome!");
    // println!("{}", s);

    // let s: &str = "This is a string slice";
    // println!("{}", s);

    // let mut s = String::from("Hello");
    // s.push('!'); // Appends a single character
    // println!("{}", s); // Output: Hello!

    // let mut s = String::from("Hello");
    // s.push_str(" Rust!"); // Appends a string
    // println!("{}", s); // Output: Hello Rust!

    // let s1 = String::from("Hello");
    // let s2 = String::from(" Rust");
    // let s3 = s1 + &s2; // s1 is moved and can't be used anymore
    // println!("{}", s3); // Output: Hello Rust

    // let s1 = String::from("Hello");
    // let s2 = String::from("Rust");
    // let s3 = format!("{} {}", s1, s2); // Doesn't take ownership
    // println!("{}", s3); // Output: Hello Rust

    // let s = String::from("Hello");
    // for c in s.chars() {
    //     println!("{}", c);
    // }

    // let s = String::from("Hello");
    // for b in s.bytes() {
    //     println!("{}", b);
    // }

    // let s = String::from("Hello, Rust!");
    // let slice = &s[0..5]; // "Hello"
    // println!("{}", slice);

    // let s = String::from("Rust");
    // print_string(&s); // Borrowing (No Ownership Transfer)
    // println!("{}", s); // Still usable

    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 85);

    println!("{:?}", scores);

    let names = vec!["Alice", "Bob"];
    let scores = vec![90, 85];

    let scores_map: HashMap<_, _> = names.iter().zip(scores.iter()).collect();
    println!("{:?}", scores_map);

    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);

    let score = scores.get("Alice"); // Returns an Option<&i32>
    match score {
        Some(value) => println!("Alice's score: {}", value),
        None => println!("No score found"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Alice"), 95);
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    println!("{:?}", scores);
    scores.entry(String::from("Bob")).or_insert(85);
    scores.entry(String::from("Alice")).or_insert(100); // Won't change since "Alice" exists

    println!("{:?}", scores); // {"Alice": 90, "Bob": 85}

    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.remove("Alice");

    println!("{:?}", scores); // Output: {}
}

// fn print_string(s: &String) {
//     println!("{}", s);
// }

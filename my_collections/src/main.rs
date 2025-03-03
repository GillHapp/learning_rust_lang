#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    println!("Hello, world!");
    let mut numbers: Vec<i32> = Vec::new(); // Create an empty vector
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("{:?}", numbers); // Output: [1, 2, 3]

    // Create a vector with elements using macros

    let mut numbers = vec![10, 20, 30, 40];
    println!("{:?}", numbers); // Output: [10, 20, 30, 40]
    println!("{}", numbers[1]); // Output: 10

    match numbers.get(2) {
        Some(value) => println!("The value is {}", value),
        None => println!("No value found"),
    }

    for num in &numbers {
        println!("{}", num);
    }

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled); // Output: [20, 40, 60, 80]

    numbers.push(4); // Add 4 at the end
    println!("{:?}", numbers); // Output: [1, 2, 3, 4]

    numbers.pop(); // Removes last element
    println!("{:?}", numbers); // Output: [1, 2, 3]

    // let first = &numbers[0]; // Immutable borrow
    // println!("{}", first);
    // numbers.push(4); //
    // println!("{:?}", numbers); // Output:

    // alternate

    let mut numbers = vec![1, 2, 3];

    if let Some(first) = numbers.get(0) {
        println!("First element: {}", first);
    }

    numbers.push(4); // ✅ No issue

    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 25,
        },
        Person {
            name: "Bob".to_string(),
            age: 30,
        },
    ];

    for person in &people {
        println!("{:?}", person);
    }
}

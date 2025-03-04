// struct PointInt {
//     x: i32,
//     y: i32,
// }

// struct PointFloat {
//     x: f64,
//     y: f64,
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

enum Option<T> {
    Some(T),
    None,
}

struct Point<T, U> {
    x: T,
    y: U,
}

use std::fmt::Display;

fn print<T: Display>(value: T) {
    println!("{}", value);
}

fn compare<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} is greater", a);
    } else {
        println!("{} is greater", b);
    }
}

fn compare2<T, U>(a: T, b: U)
where
    T: Display + PartialOrd,
    U: Display + PartialOrd,
{
    println!("{} and {}", a, b);
}

fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", largest(10, 20)); // Works with i32
    println!("{}", largest(10.5, 5.2)); // Works with f64

    // let p1 = Point { x: 10, y: 20 }; // i32
    // let p2 = Point { x: 10.5, y: 20.2 }; // f64

    // println!("p1: ({}, {})", p1.x, p1.y);
    // println!("p2: ({}, {})", p2.x, p2.y);

    let p1 = Point { x: 10, y: 20.5 }; // x: i32, y: f64
    let p2 = Point { x: "Rust", y: 3.14 }; // x: &str, y: f64

    let some_number = Some(5); // i32
    println!("{:?}", some_number);
    let some_text = Some("Hello"); // &str
    println!("{:?}", some_text);

    print(42); // Works with i32
    print("Hello, Rust!"); // Works with &str
}
// fn largest_i32(a: i32, b: i32) -> i32 {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }

// fn largest_f64(a: f64, b: f64) -> f64 {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
// Variables and Mutability
const MAX_LIMIT: i32 = 100; // Constant (must be typed and immutable)
const PI: f64 = 3.1415926535;

fn main() {
    let mut x = 5; // Mutable variable
    println!("Initial value of x: {x}");

    x += 1; // Mutating x
    println!("After mutation, x: {x}");

    {
        let x = x * 2; // Shadowing: creates a new x inside this scope
        println!("Inner scope x: {x}");
    }

    println!("Outer scope x after inner block: {x}");

    // Shadowing again with a different type
    let x = "Rust"; // Now x is a string
    println!("Shadowed x as string: {x}");

    // Working with tuples and destructuring
    let (a, b, c) = (10, 3.14, "Hello");
    println!("Tuple values: a = {a}, b = {b}, c = {c}");

    // Using mutable arrays
    let mut numbers = [1, 2, 3, 4, 5];
    numbers[0] = 99; // Modifying array
    println!("Modified array: {:?}", numbers);

    // Using a loop to modify values
    for num in &mut numbers {
        *num *= 2;
    }
    println!("Doubled array values: {:?}", numbers);

    println!("MAX_LIMIT constant: {MAX_LIMIT}");

    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Scalar Types
    let integer: i32 = 42; // Signed integer
    let unsigned_integer: u8 = 255; // Unsigned integer
    let float: f64 = 3.14; // Floating-point number
    let boolean: bool = true; // Boolean
    let character: char = 'R'; // Character type

    println!("Integer: {integer}, Unsigned: {unsigned_integer}, Float: {float}");
    println!("Boolean: {boolean}, Character: {character}");

    // Tuple - fixed-size collection of different types
    let my_tuple: (i32, f64, char) = (10, 6.28, 'T');
    let (x, y, z) = my_tuple; // Destructuring
    println!("Tuple values: x = {x}, y = {y}, z = {z}");

    // Arrays - fixed-size collection of the same type
    let my_array: [i32; 3] = [1, 2, 3]; // Array of 3 integers
    println!("Array: {:?}", my_array);

    // Slices - a reference to a portion of an array
    let slice: &[i32] = &my_array[0..2]; // Slice of first 2 elements
    println!("Slice: {:?}", slice);

    // Vector - dynamic, growable array
    let mut my_vector: Vec<i32> = vec![10, 20, 30];
    my_vector.push(40);
    println!("Vector: {:?}", my_vector);

    // Struct - custom data type
    struct Person {
        name: String,
        age: u8,
    }
    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };
    println!("Struct -> Name: {}, Age: {}", person.name, person.age);

    // Enum - type with multiple variants
    enum Color {
        Red,
        Green,
        Blue,
        Custom(String),
    }
    let my_color = Color::Custom(String::from("Purple"));

    match my_color {
        Color::Red => println!("Color: Red"),
        Color::Green => println!("Color: Green"),
        Color::Blue => println!("Color: Blue"),
        Color::Custom(color) => println!("Custom color: {color}"),
    }

    // Using constants
    println!("Constant PI: {PI}");
}

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
// Variables and Mutability
const MAX_LIMIT: i32 = 100; // Constant (must be typed and immutable)
const PI: f64 = 3.1415926535;

fn greet() {
    println!("Hello, Rustacean!");
}

// 2. Function with parameters
fn add(a: i32, b: i32) {
    println!("Sum: {}", a + b);
}

// 3. Function with a return value
fn multiply(a: i32, b: i32) -> i32 {
    a * b // No semicolon means it's an implicit return
}

// 4. Function with an explicit return statement
fn divide(a: f64, b: f64) -> f64 {
    return a / b;
}

// 5. Function with multiple return values (using a tuple)
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

// 6. Function with mutable reference parameter
fn increase_by_ten(num: &mut i32) {
    *num += 10;
}

// 7. Function using generic types
fn generic_max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 8. Function with a closure as a parameter
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

// 9. Function that returns another function (higher-order function)
fn make_multiplier(multiplier: i32) -> impl Fn(i32) -> i32 {
    move |x| x * multiplier
}

// 10. Recursive function (calls itself)
fn factorial(n: u32) -> u32 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

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
    let _red = Color::Red;
    let _green = Color::Green;
    let _blue = Color::Blue;

    match my_color {
        Color::Red => println!("Color: Red"),
        Color::Green => println!("Color: Green"),
        Color::Blue => println!("Color: Blue"),
        Color::Custom(color) => println!("Custom color: {color}"),
    }

    // Using constants
    println!("Constant PI: {PI}");

    // Calling basic functions
    greet();
    add(3, 5);

    let result = multiply(4, 6);
    println!("Multiplication result: {result}");

    let quotient = divide(10.0, 2.0);
    println!("Division result: {quotient}");

    let (new_a, new_b) = swap(10, 20);
    println!("Swapped values: new_a = {new_a}, new_b = {new_b}");

    // Mutable reference example
    let mut value = 15;
    increase_by_ten(&mut value);
    println!("Value after increase: {value}");

    // Using generic function
    let max_value = generic_max(10, 20);
    println!("Max value: {max_value}");

    // Using closures with a function
    let square = |x: i32| x * x;
    let squared_value = apply(square, 4);
    println!("Squared value: {squared_value}");

    // Using a function that returns a function
    let triple = make_multiplier(3);
    println!("Triple of 5: {}", triple(5));

    // Recursive function call
    let fact = factorial(5);
    println!("Factorial of 5: {fact}");

    // Control Flow

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // if we do it like that

    // let condition = true;

    // let number = if condition { 5 } else { "six" };  // this is wrong because the types are different

    // println!("The value of number is: {number}");

    // Loops

    // loop {
    //     println!("again!");
    // }                   // loop again and again and again and again

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let number = 2;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"), // _ means "anything else"
    }

    let num = 4;

    match num {
        1 | 2 => println!("One or Two"),    // Multiple matches
        3..=5 => println!("Three to Five"), // Range match
        _ => println!("Other"),
    }

    let point = (3, 5);

    match point {
        (0, y) => println!("On Y-axis at {y}"),
        (x, 0) => println!("On X-axis at {x}"),
        (x, y) => println!("Point at ({x}, {y})"),
    }

    let some_number = Some(5); // some_number is an Option<i32> with value Some(5)

    if let Some(x) = some_number {
        println!("Matched: {x}");
    } else {
        println!("No match");
    }

    // alternative solution

    let some_number = Some(5);

    match some_number {
        Some(x) => println!("Matched: {x}"),
        None => println!("No match"),
    }
}

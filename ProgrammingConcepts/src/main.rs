
// Variables and Mutability
const MAX_LIMIT: i32 = 100; // Constant (must be typed and immutable)

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
}


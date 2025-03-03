// Import modules from library.rs and utils directory
mod library;
mod utils;

use library::greet;
use utils::logger;
use utils::math::add; // Bringing specific function into scope // Bringing entire module into scope

fn main() {
    let a = 10;
    let b = 5;

    let sum = add(a, b);
    println!("Sum: {}", sum);

    logger::log_info("Program executed successfully!");
    greet("Rust");
}

use std::fmt::Display;

// 🔹 Struct with a Lifetime Parameter
#[derive(Debug)]
struct Book<'a> {
    title: &'a str,  // Reference with lifetime 'a
    author: &'a str, // Reference with lifetime 'a
}

// 🔹 Function that takes two references and returns the longer one
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 🔹 Struct Implementing a Trait with Lifetimes
trait Printable {
    fn print(&self);
}

impl<'a> Printable for Book<'a> {
    fn print(&self) {
        println!("📖 Book: '{}' by {}", self.title, self.author);
    }
}

// 🔹 Function with Lifetime Elision (Rust infers the lifetime automatically)
fn greet(name: &str) -> &str {
    println!("Hello, {name}!");
    name // Rust infers lifetime automatically
}

// 🔹 Static Lifetime: Reference that lives for the entire program duration
fn static_lifetime() -> &'static str {
    "This is a string literal with 'static lifetime."
}

fn main() {
    // Example: Using lifetimes in functions
    let text1 = String::from("Rust is amazing!");
    let text2 = String::from("Rust is powerful!");
    let result = longest(&text1, &text2);
    println!("📌 The longest text is: {}", result);

    // Example: Struct with lifetimes
    let book_title = String::from("The Rust Programming Language");
    let book_author = String::from("Steve Klabnik & Carol Nichols");

    let my_book = Book {
        title: &book_title,
        author: &book_author,
    };

    // Using trait with lifetime
    my_book.print();

    // Using a function with lifetime elision
    let user = greet("Alice");
    println!("✅ Greeting: {user}");

    // Using static lifetime
    let static_str = static_lifetime();
    println!("🚀 Static Lifetime String: {}", static_str);
}

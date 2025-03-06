// // Defining a trait
// trait Speak {
//     fn speak(&self); // Trait method (must be implemented by types)
// }

// // Implementing the trait for a struct
// struct Dog;
// struct Cat;

// impl Speak for Dog {
//     fn speak(&self) {
//         println!("Woof! 🐶");
//     }
// }

// impl Speak for Cat {
//     fn speak(&self) {
//         println!("Meow! 🐱");
//     }
// }

// fn main() {
//     let dog = Dog;
//     let cat = Cat;

//     dog.speak(); // Output: Woof! 🐶
//     cat.speak(); // Output: Meow! 🐱
// }

// trait Greet {
//     fn greet(&self) {
//         println!("Hello! 👋");
//     }
// }

// struct Human;
// impl Greet for Human {} // Uses the default implementation

// fn main() {
//     let person = Human;
//     person.greet(); // Output: Hello! 👋
// }

// trait Show {
//     fn show(&self);
// }

// struct Number(i32);
// struct Stringy(String);

// impl Show for Number {
//     fn show(&self) {
//         println!("Number: {}", self.0);
//     }
// }
// impl Show for Stringy {
//     fn show(&self) {
//         println!("String: {}", self.0);
//     }
// }

// Function that only accepts types implementing Show
// fn print_value<T: Show>(item: T) {
//     item.show();
// }
// fn print_value(item: impl Show) {
//     item.show();
// }

// fn main() {
//     let num = Number(42);
//     let str = Stringy("Hello, Rust!".to_string());

//     print_value(num); // Output: Number: 42
//     print_value(str); // Output: String: Hello, Rust!
// }
// fn main() {
//     let num = Number(42);
//     print_value(num); // Output: Number: 42
// }

trait A {
    fn a_method(&self);
}

trait B {
    fn b_method(&self);
}

struct Example;
impl A for Example {
    fn a_method(&self) {
        println!("A method");
    }
}
impl B for Example {
    fn b_method(&self) {
        println!("B method");
    }
}

// Function requiring both A and B traits
fn use_both<T: A + B>(item: T) {
    item.a_method();
    item.b_method();
}

fn main() {
    let ex = Example;
    use_both(ex);
}

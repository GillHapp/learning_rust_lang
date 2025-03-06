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

trait Greet {
    fn greet(&self) {
        println!("Hello! 👋");
    }
}

struct Human;
impl Greet for Human {} // Uses the default implementation

fn main() {
    let person = Human;
    person.greet(); // Output: Hello! 👋
}

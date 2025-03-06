// Defining a trait
trait Speak {
    fn speak(&self); // Trait method (must be implemented by types)
}

// Implementing the trait for a struct
struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof! 🐶");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow! 🐱");
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    dog.speak(); // Output: Woof! 🐶
    cat.speak(); // Output: Meow! 🐱
}

struct Point<T, U> {
    x: T,
    y: U,
}

// Generic method that combines two different types
impl<T, U> Point<T, U> {
    fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 }; // Point<i32, i32>
    let p2 = Point {
        x: "Hello",
        y: "Rust",
    }; // Point<&str, &str>

    let mixed = p1.mix(p2); // mixed: Point<i32, &str>
    println!("Mixed Point: ({}, {})", mixed.x, mixed.y);
}

#[derive(Debug)] // Enables debug printing using {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if the current rectangle can completely contain another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // A method to display the rectangle's details
    fn display(&self) {
        println!(
            "Rectangle: Width = {}, Height = {}, Area = {}",
            self.width,
            self.height,
            self.area()
        );
    }
}

fn main() {
    // Creating three Rectangle instances with different dimensions
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Display the rectangles' properties
    println!("📌 Rectangle Details:");
    rect1.display();
    rect2.display();
    rect3.display();

    // Checking if rect1 can hold rect2 and rect3
    println!(
        "\n✅ Can Rectangle 1 hold Rectangle 2? 👉 {}",
        if rect1.can_hold(&rect2) {
            // self from rect1 and other from rect2
            "Yes ✅"
        } else {
            "No ❌"
        }
    );

    println!(
        "✅ Can Rectangle 1 hold Rectangle 3? 👉 {}",
        if rect1.can_hold(&rect3) {
            "Yes ✅"
        } else {
            "No ❌"
        }
    );
}

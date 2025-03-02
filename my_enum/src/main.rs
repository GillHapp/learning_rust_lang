#[derive(Debug)] // Enables printing with {:?}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Message {
    Text(String),
    Coordinates(i32, i32),
    Exit,
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 30,
        }
    }
}

// Function returning Option<T>
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

// Function returning Result<T, E>
fn square_root(number: f64) -> Result<f64, String> {
    if number < 0.0 {
        Err(String::from(
            "Cannot calculate square root of a negative number",
        ))
    } else {
        Ok(number.sqrt())
    }
}

fn main() {
    // 1️⃣ Enum for Directions
    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Moving Up ⬆️"),
        Direction::Down => println!("Moving Down ⬇️"),
        Direction::Left => println!("Moving Left ⬅️"),
        Direction::Right => println!("Moving Right ➡️"),
    }

    // 2️⃣ Enum with Data (Message)
    let msg = Message::Text(String::from("Hello, Enum!"));
    match msg {
        Message::Text(content) => println!("Received message: {}", content),
        Message::Coordinates(x, y) => println!("Coordinates: ({}, {})", x, y),
        Message::Exit => println!("Exiting program..."),
    }

    // 3️⃣ Enum with Methods (TrafficLight)
    let light = TrafficLight::Red;
    println!("{:?} light stays for {} seconds.", light, light.duration());

    // 4️⃣ Using Option<T> Enum
    let result = divide(10.0, 2.0);
    match result {
        Some(value) => println!("Division Result: {}", value),
        None => println!("Cannot divide by zero!"),
    }

    // 5️⃣ Using Result<T, E> Enum for Error Handling
    let sqrt_result = square_root(-9.0);
    match sqrt_result {
        Ok(value) => println!("Square root: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}

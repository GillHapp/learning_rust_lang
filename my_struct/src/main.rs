struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };


    // Using Tuple Structs Without Named Fields to Create Different Types

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
   


}
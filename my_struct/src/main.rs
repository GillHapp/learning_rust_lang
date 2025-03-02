#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// unit like struct

struct UnitLikeStruct;

struct User {
    name: String, // Owned data (String stores data on the heap)
    age: u32,
}

struct User<'a> {
    name: &'a str, // Borrowed data (does not own the string)
    age: u32,
}

struct User {
    name: String, // Owned String
}

fn print_user(user: User) { // Takes ownership
    println!("User: {}", user.name);
} // `user` is dropped here!

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
   


    // Unit-Like Structs Without Any Fields
       
    let unit_like_struct = UnitLikeStruct;

    let user1 = User {
        name: String::from("Alice"), // Ownership moved into `user1`
        age: 25,
    };

    let user2 = user1; // ❌ Ownership moved, `user1` is no longer valid

    // println!("{}", user1.name); // ❌ ERROR: `user1` is moved
    println!("{}", user2.name); // ✅ Works fine

    let name = String::from("Alice"); // Owned by `name`
    let user = User { name: &name, age: 25 }; // Borrowing `name`

    println!("User's name: {}", user.name); // ✅ Works, no ownership move



    // 3️⃣ Transferring Ownership with Functions

    let user = User {
        name: String::from("Alice"),
    };

    print_user(user); // Ownership moved to function
    // println!("{}", user.name); // ❌ ERROR: `user` is moved
}
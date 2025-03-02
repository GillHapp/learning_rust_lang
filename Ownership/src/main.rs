fn main() {
    let x = String::from("Hello, Rust!"); // `x` owns the string
    println!("{}", x); // ✅ This works

    let y = x; // Ownership moves from `x` to `y`

    // println!("{}", x); ❌ ERROR: `x` is no longer valid
    println!("{}", y); // ✅ Only `y` can be used now

    let x = String::from("Hello, Rust!");
    let y = x.clone(); // ✅ `y` gets a deep copy

    println!("{}", x); // ✅ Now both `x` and `y` are valid
    println!("{}", y);

    // let msg = String::from("Rust is awesome!");
    // print_message(msg);

    // : Borrowing (&) Instead of Moving

    // Instead of transferring ownership, we can borrow a value using references (&).

    // let msg = String::from("Rust is amazing!");
    // print_message(&msg); // ✅ Pass a reference (borrow)

    // println!("{}", msg); // ✅ Still valid because ownership wasn't moved

    // Mutable Borrowing (&mut)

    // let mut msg = String::from("Rust is fast");
    // add_exclamation(&mut msg); // ✅ Mutably borrow msg

    // println!("{}", msg); // ✅ Now it has "!!!"

    // Rules of Mutable Borrowing
    // ✔ Only one mutable reference at a time (prevents race conditions).
    // ✔ No immutable (&) and mutable (&mut) references at the same time.

    // ❌ This will cause an error:

    // rust
    // Copy
    // Edit
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // ❌ ERROR: Second mutable borrow not allowed

    // Slices - Avoiding Borrowing Issues

    let s = String::from("hello world");
    let word = first_word(&s);

    println!("First word: {}", word);

    // slices

    // converting string onto array of bytes with string.as_bytes() so that we can able to iterate over it

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word  = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn print_message(message: String) {
    // `message` takes ownership
    println!("{}", message);
}

// fn print_message(message: &String) {
//     // Borrowing, no ownership transfer
//     println!("{}", message);
// }

fn add_exclamation(s: &mut String) {
    // Mutable borrow
    s.push_str("!!!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // Return slice of the first word
        }
    }

    &s // If no space found, return the whole string
}
// this is slices of bytes

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..] // this is for return then whole string if there is no space in string
// }

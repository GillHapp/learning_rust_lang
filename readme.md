We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

When you run `cargo build --release`, Rust compiles your code with optimizations to make it run faster. However, this takes more time to compile compared to the default `cargo build`, which is used for development and compiles quickly but without optimizations.  

In short:  
- **`cargo build`** → Fast compilation, slower execution (for development).  
- **`cargo build --release`** → Slow compilation, faster execution (for final release).  

Key Concepts Used:
Constants (const) – Always immutable and require type annotations.
Mutability (mut) – Allows modifying a variable.
Shadowing – Reusing a variable name but changing its type or value.
Tuples – Multiple values in one variable, accessed by destructuring.
Arrays & Loops – Modifying arrays using a loop.

let guess: u32 = "42".parse().expect("Not a number!");
If we don’t add the : u32 type annotation shown in the preceding code, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:

Scalar Types
A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.

Function 

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value.

Types of Functions Demonstrated:
Basic function (greet()) – No parameters, no return.
Function with parameters (add(a, b)) – Takes input, prints result.
Function with a return value (multiply(a, b)) – Returns a computed value.
Function with an explicit return statement (divide(a, b)) – Uses return keyword.
Function returning multiple values (swap(a, b)) – Uses tuples.
Function with mutable reference (increase_by_ten(&mut num)) – Modifies an argument.
Generic function (generic_max<T>(a, b)) – Works with multiple data types.
Function taking a closure (apply(f, value)) – Accepts another function as input.
Higher-order function (make_multiplier(multiplier)) – Returns a function.
Recursive function (factorial(n)) – Calls itself to compute results.


🎯 Summary of Rust Control Flow
Control Flow	Usage
if, else if, else	Conditional execution
if let	Match a single pattern concisely
while	Loop while a condition is true
while let	Loop while a pattern matches
for	Iterate over ranges, arrays, and iterators
loop	Infinite loop, exit with break
match	Pattern matching (powerful alternative to if-else)
break	Exit a loop
continue	Skip an iteration
Labeled loops ('label:)	Exit specific loops in nested loops


 Rules of Mutable Borrowing
✔ Only one mutable reference at a time (prevents race conditions).
✔ No immutable (&) and mutable (&mut) references at the same time.

❌ This will cause an error:

rust
Copy
Edit
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // ❌ ERROR: Second mutable borrow not allowed

let mut s = String::from("hello");

{
    let r1 = &mut s; // ✅ Borrow only within this block
}

let r2 = &mut s; // ✅ Allowed because r1 is no longer used

Summary of Ownership in Rust
Concept	Explanation
Ownership	Each value has one owner. When the owner goes out of scope, memory is freed.
Move	Assigning a variable to another moves ownership (String).
Clone	Creates a new copy instead of moving ownership.
Borrowing (&)	Allows reading a value without taking ownership.
Mutable Borrowing (&mut)	Allows modifying a value but only one mutable reference at a time.
Slices (&[T])	References part of a collection without ownership transfer.

# **📜 Rust Ownership, Borrowing, and References – All Rules**  

Rust ensures **memory safety** at **compile time** using ownership, borrowing, and references. Below are **all the rules** you need to know! 🚀  

---

# **🔹 Ownership Rules**
1️⃣ **Each value in Rust has a single owner** (only one variable owns a value at a time).  
2️⃣ **When the owner goes out of scope, the value is dropped** (memory is freed).  
3️⃣ **Values can be moved or copied, but not both.**
   - **Move**: Ownership is transferred.
   - **Copy**: Creates a duplicate (only for types that implement `Copy` trait, like integers).
4️⃣ **A value can only have one owner at a time**—once moved, the original variable is invalid.

### **✅ Example: Move vs Copy**
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // Moves ownership from s1 to s2

    // println!("{}", s1); ❌ ERROR: s1 is invalid after move
    println!("{}", s2); // ✅ Works because s2 owns the data now

    let x = 5;
    let y = x; // Copy, because integers implement Copy trait
    println!("{}", x); // ✅ Works because Copy does not move ownership
}
```

---

# **🔹 Borrowing Rules**
Borrowing allows us to reference a value **without taking ownership**.

1️⃣ **Immutable references (`&T`) allow multiple borrows at the same time.**  
2️⃣ **Mutable references (`&mut T`) allow only one borrow at a time.**  
3️⃣ **Cannot mix mutable (`&mut T`) and immutable (`&T`) references to the same variable at the same time.**  
4️⃣ **References must always be valid** (they cannot outlive the borrowed value).

### **✅ Example: Immutable Borrowing**
```rust
fn main() {
    let s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // ✅ Multiple immutable borrows allowed

    // let r3 = &mut s; ❌ ERROR: Cannot have a mutable reference when immutable ones exist
}
```

### **✅ Example: Mutable Borrowing**
```rust
fn main() {
    let mut s = String::from("Hello");

    let r1 = &mut s; // ✅ Allowed
    // let r2 = &mut s; ❌ ERROR: Only one mutable borrow at a time
}
```

### **✅ Example: No Mixing Mutable and Immutable Borrows**
```rust
fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; ❌ ERROR: Cannot have a mutable borrow when immutable ones exist

    println!("{} and {}", r1, r2);
}
```

✅ **Solution**: Mutable borrow after immutable ones are done
```rust
fn main() {
    let mut s = String::from("Hello");

    {
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2); // ✅ Immutable references used here
    } // r1 and r2 go out of scope

    let r3 = &mut s; // ✅ Allowed because r1 and r2 are no longer used
}
```

---

# **🔹 Reference Rules**
References allow us to use a value **without taking ownership**.

1️⃣ **References must always be valid** (Rust prevents dangling references).  
2️⃣ **Cannot return a reference to a variable that goes out of scope** (prevents use-after-free errors).  
3️⃣ **String slices (`&str`) and array slices (`&[T]`) allow safe partial access.**

### **✅ Example: Preventing Dangling References**
```rust
fn dangle() -> &String { // ❌ ERROR: This function returns a dangling reference
    let s = String::from("Hello");
    &s // ❌ `s` will be dropped at the end of this function
}
```
✅ **Solution: Return the owned value instead**
```rust
fn no_dangle() -> String {
    let s = String::from("Hello");
    s // ✅ Ownership is returned
}
```

---

# **🔹 Summary of All Rules**
| **Concept** | **Rules** |
|------------|----------|
| **Ownership** | 1. Each value has only **one owner**. <br> 2. When the owner goes out of scope, the value is **dropped**. <br> 3. Ownership can be **moved** (transferred to another variable). <br> 4. Values that implement `Copy` can be **duplicated** instead of moved. |
| **Borrowing** | 1. Immutable references (`&T`) allow **multiple** borrows. <br> 2. Mutable references (`&mut T`) allow **only one borrow at a time**. <br> 3. Cannot mix **mutable and immutable references** at the same time. <br> 4. A borrowed value must always be **valid**. |
| **References** | 1. References must **not outlive** the data they refer to. <br> 2. Cannot return a reference to a value that goes out of scope. <br> 3. String slices and array slices allow **safe, partial access**. |

---

# **🎯 Key Takeaways**
✅ Rust **prevents memory leaks** and **data races** using these rules.  
✅ Borrowing allows **safe access** without taking ownership.  
✅ **Mutable borrowing** is strict but prevents bugs.  
✅ **References** must always be valid (Rust ensures no use-after-free errors).  

🚀 **Mastering ownership, borrowing, and references will make you a Rust pro!** Do you need more examples or explanations? 😊
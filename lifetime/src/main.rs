fn get_reference() -> &i32 {
    // ❌ ERROR: `x` will be dropped at the end of function
    let x = 10;
    &x // Returns a reference to `x`, but `x` will be dropped! 💥
}
fn main() {
    let r = get_reference();
    println!("{}", r); // ⚠️ ERROR: Use of a dangling reference!
}

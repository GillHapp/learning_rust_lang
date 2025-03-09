pub mod modname {
    pub fn make_multiplier(multiplier: i32) -> impl Fn(i32) -> i32 {
        move |x| x * multiplier
    }
}

use crate::modname::make_multiplier;
fn main() {
    let double = make_multiplier(2);
    println!("{}", double(5)); // Output: 10
}

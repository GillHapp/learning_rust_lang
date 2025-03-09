fn apply<F: Fn(i32)>(f: F, x: i32) {
    f(x);
}

fn main() {
    let print_num = |num| println!("Number: {num}");
    apply(print_num, 10);
}

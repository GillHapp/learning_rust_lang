fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    println!("Got: {:?}", v1)



    let mut numbers = vec![1, 2, 3];

    for num in numbers.iter_mut() {
        *num *= 2;
    }

    println!("{:?}", numbers); // Output: [2, 4, 6]


    let numbers = vec![1, 2, 3];

    for num in numbers.into_iter() {
        println!("{}", num);
    }

    // println!("{:?}", numbers); ❌ ERROR: numbers is moved
}

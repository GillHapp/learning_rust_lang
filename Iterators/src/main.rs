fn main() {
    // let v1 = vec![1, 2, 3];

    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {val}");
    // }

    // println!("Got: {:?}", v1);

    // let mut numbers = vec![1, 2, 3];

    // for num in numbers.iter_mut() {
    //     *num *= 2;
    // }

    // println!("{:?}", numbers); // Output: [2, 4, 6]

    // let numbers = vec![1, 2, 3];

    // for num in numbers.into_iter() {
    //     println!("{}", num);
    // }

    // println!("{:?}", numbers); ❌ ERROR: numbers is moved

    let numbers = vec![10, 20, 30]; // Store vector in a variable

    let mut iter = numbers.iter(); // Now iter borrows from `numbers`, which lives longer

    println!("{:?}", iter.next()); // Some(10)
    println!("{:?}", iter.next()); // Some(20)
    println!("{:?}", iter.next()); // Some(30)

    // alternate method for the same

    let mut iter = vec![10, 20, 30].into_iter(); // Moves ownership

    println!("{:?}", iter.next()); // Some(10)
    println!("{:?}", iter.next()); // Some(20)
    println!("{:?}", iter.next()); // Some(30)
}

// important note ## // If you reset `iter`, it starts again

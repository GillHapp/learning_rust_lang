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

    // let numbers = vec![10, 20, 30]; // Store vector in a variable

    // let mut iter = numbers.iter(); // Now iter borrows from `numbers`, which lives longer

    // println!("{:?}", iter.next()); // Some(10)
    // println!("{:?}", iter.next()); // Some(20)
    // println!("{:?}", iter.next()); // Some(30)

    // // alternate method for the same

    // let mut iter = vec![10, 20, 30].into_iter(); // Moves ownership

    // println!("{:?}", iter.next()); // Some(10)
    // println!("{:?}", iter.next()); // Some(20)
    // println!("{:?}", iter.next()); // Some(30)

    let numbers = vec![1, 2, 3];

    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();

    println!("{:?}", doubled); // [2, 4, 6]

    let numbers = vec![1, 2, 3, 4, 5];

    let even_numbers: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();

    println!("{:?}", even_numbers); // [2, 4]

    let numbers = vec![1, 2, 3, 4, 5];

    let first_even = numbers.iter().find(|&&x| x % 2 == 0);

    println!("{:?}", first_even); // Some(2)

    let numbers = vec![1, 2, 3, 4, 5];

    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_even = numbers.iter().all(|&x| x % 2 == 0);

    println!("Has even? {}", has_even); // true
    println!("All even? {}", all_even); // false

    let numbers = vec![1, 2, 3, 4, 5];

    let sum = numbers.iter().fold(0, |acc, &x| acc + x);

    println!("{}", sum); // Output: 15

    let letters = vec!["a", "b", "c"];

    for (index, letter) in letters.iter().enumerate() {
        println!("Index: {}, Letter: {}", index, letter);
    }
}

// important note ## // If you reset `iter`, it starts again

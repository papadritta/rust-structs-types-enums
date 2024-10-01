fn get_item(index: usize) {
    // //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn sum_of_elements(vec: &Vec<i32>) -> i32 {
    let sum: i32 = vec.iter().sum();
    sum
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    // Retrieve a value at a specific index
    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"), // Added code to handle empty vector
    }

    let sum = sum_of_elements(&vec);
    println!("The sum of all elements in the vector is: {}", sum);
}

// Modify the code to handle the case when the vector is empty. Uncomment and complete the code inside the `match` statement in the `main` function to print a specific message when the vector is empty.
// Extend the program by implementing a function that takes a vector as input and returns the sum of all its elements. Invoke this function with the `vec` variable in the `main` function and print the sum.

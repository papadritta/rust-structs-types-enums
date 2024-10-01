// fn main() {
//     let mut v = vec![1, 2, 3];
//     v.push(4);
//     println!("{:?}", v); // Output: [1, 2, 3, 4]

//     // extend adds each element of the given slice to the vector
//     let more_numbers = vec![5, 6];
//     v.extend(more_numbers);
//     println!("{:?}", v);

//     // append adds the given vector to the vector, requires the vector to be mutable
//     let mut other_numbers = vec![7, 8];
//     v.append(&mut other_numbers);
//     println!("{:?}", v);

//     // insert items at a given index
//     v.insert(0, 0);
//     println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]
// }

fn insert_value_at_ends(vector: &mut Vec<i32>, value: i32) {
    vector.insert(0, value);
    vector.push(value);
}

fn append_vectors(first: &mut Vec<i32>, second: Vec<i32>) {
    first.extend(second);
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    insert_value_at_ends(&mut v, 5);
    println!("{:?}", v); // Output: [5, 1, 2, 3, 4, 5]

    let more_numbers = vec![6, 7];
    v.extend(more_numbers);
    println!("{:?}", v);

    let mut other_numbers = vec![8, 9];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9]

    let additional_numbers = vec![10, 11, 12];
    append_vectors(&mut v, additional_numbers);
    println!("{:?}", v); // Output: [0, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
}

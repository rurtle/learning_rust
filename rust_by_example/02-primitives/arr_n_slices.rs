use std::mem;

fn main() {
    // Fixed size array. Type signature is superfluous
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized with the same value
    let ys: [i32; 500] = [23; 500];

    // Arary indexing starts at 0
    println!("First element: {}", ys[0]);
    println!("Second element: {}", ys[1]);

}
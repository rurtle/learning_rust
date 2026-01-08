use std::mem;

// This function borrows a slice
fn inspect_slice(slice: &[i32]) {
    println!("First element: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed size array. Type signature is superfluous
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized with the same value
    let ys: [i32; 500] = [23; 500];

    // Arary indexing starts at 0
    println!("First element: {}", ys[0]);
    println!("Second element: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("Number of elements in xs: {}", xs.len());
    println!("Length of the array ys: {}", ys.len());

    // Arrays are stack allocated
    println!("Size of xs: {} bytes", mem::size_of_val(&xs));
    println!("Size of ys: {} bytes", mem::size_of_val(&ys));

    // Arrays can be automatically borrowed as slices
    inspect_slice(&xs);
    inspect_slice(&ys[0..100]);

    // Example of empty slice
    let empty_arr: [u32; 0] = [];
    assert_eq!(&empty_arr, &[]);
    assert_eq!(&empty_arr, &[][..]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with `.expect()`
    // if you would like the program to exit with a nice message instead of
    // happily continuing
    for i in 0..xs.len() + 1{
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far", i),
        }
    }

    // out of bound indexing on array with constant value causes compile
    // time error
    // println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error
    println!("{}", xs[..][7]);
}
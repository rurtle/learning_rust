fn main() {
    // This binding lives in the main function
    let long_lived_binding = 5.0f32;

    // This is a block and has a smaller scope than the main function
    {
        // This binding only exists within this block
        let short_lived_binding = 2.0f32;

        println!("inner short: {}", short_lived_binding);
    }
    // End of block

    // Error! Accessing short_lived_binding will give error
    // println!("outer short: {}", short_lived_binding);

    // it's alright to access the long_lived_binding here
    println!("Outer long_lived_binding: {}", long_lived_binding);
}
fn main() {
    // Positional arguments in formattting
    println!("{} days in a month", 31);

    // Positional arguments - some more of it
    println!("{}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over");

    // Formatting to represent the same number under different bases
    println!("Base 10:                      {}", 9999999);
    println!("Base 2:                      {:b}", 9999999);
    println!("Base 8:                      {:o}", 9999999);
    println!("Base 16:                      {:x}", 9999999);
}

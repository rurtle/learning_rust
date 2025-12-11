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

    // Right justify with spaces
    println!("{number:>8}", number = 1);
    // Right justify with 0s
    println!("{number:0>8}", number = 1);
    // Left justify with 0s
    println!("{number:0<8}", number = 1);
    // Making it slightly more dynamic
    println!("{number:0>width$}", number = 1, width = 8);

    // Argument enumeration
    println!("My name is {1}, {0} {1}", "James", "Bond");

    let number: f64 = 1.0;
    let width: usize = 8;
    println!("{number:0>width$}");
}

fn main() {
    let shadow_binding = 1;

    {
        println!("before being shadowed: {}", shadow_binding);

        // This binding *shadows* the outer one
        let shadow_binding = "abv";

        println!("shadow in the inner block: {}", shadow_binding);
    }

    println!("Outside inner block: {}", shadow_binding);

    // This binding *shadows* the previous binding
    let shadow_binding = "234";
    println!("Shadowed in outer block: {}", shadow_binding);
}
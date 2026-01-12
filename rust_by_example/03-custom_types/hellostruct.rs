// An attribute to hide warnings for unused variables
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    sex: String,
}

// A unit struct
struct MyUnit;

// A tuple struct
struct Pair(i32, i32);

// A tuple struct with 2 fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right corners are
    // in space
    top_left: Point,
    bottom_right: Point,
}

// Calculate the area of the rectangle
fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: Point { x: left_edge, y: top_edge }, bottom_right: Point { x: right_edge, y: bottom_edge } } = rect;
    (right_edge - left_edge) * (bottom_edge - top_edge)
}

// Driver function
fn main() {
    // Create struct with field init shorthand
    let name = String::from("Takahashi Yamamoto");
    let age = 23;
    let sex = String::from("M");
    let p1 = Person { name, age, sex };

    // Print debug struct
    println!("Debug struct: {:?}", p1);

    // Instantiate a Point
    let po1: Point = Point { x: 0.3, y: 4.0 };
    let po2: Point = Point { x: 4.0, y: 0.3 };

    // Access field of the Point instance
    println!("Point coordinates: ({}, {})", po1.x, po1.y);
    println!("Point coordinates: ({}, {})", po2.x, po2.y);

    // Make a new point by using struct update syntax to use the fields of
    // the other one
    let po3 = Point { x: 7.5, ..po2 };
    println!("Debug struct: ({}, {})", po3.x, po3.y);

    // Destructure the Point struct using a `let` binding
    let Point{ x: left_edge, y: top_edge} = po1;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: po2,
    };

    println!("Top Left: ({}, {})", _rectangle.top_left.x, _rectangle.top_left.y);
    println!("Bottom right: ({}, {})", _rectangle.bottom_right.x, _rectangle.bottom_right.y);

    // Instantiate an unit struct
    let _unit = MyUnit;

    // Instantiate a tuple struct
    let pair = Pair(1, 2);

    // Access tuple struct fields
    println!("Debug struct: ({}, {})", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Exercise: Calculate area of the rectangle
    println!("Rectangle area: {}", rect_area(_rectangle));
}
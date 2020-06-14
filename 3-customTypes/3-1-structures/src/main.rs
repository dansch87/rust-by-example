/*
3 Types of structures can be created using struct keyword:
	1) Tuple structs, which are basically named tuples
	2) classic C structs
	3) Unit structs, which are field-less, are useful for generics

*/
#![allow(dead_code)] //crate-lvl allow attr




// 1) TUPLE STRUCT
struct Pair(i32, f32);


// 2) CLASSIC C STRUCT
struct Person<'a> {
    name: &'a str, // The 'a defines a lifetime
    age: u8,
}

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}


// 3) UNIT STRUCT
struct Unit;





fn main() {
    
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    
}

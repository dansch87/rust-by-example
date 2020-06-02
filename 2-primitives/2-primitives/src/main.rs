/*
	__SCALAR TYPES__
	signed integers: i8, i16, i32, i128, isize (pointer size)
	unsigned integers: u8, u16, u32, u64, u128, usize (pointer size)
	floating point: f32, f64
	char: Unicode scalar values (4 byte each)
	bool: true, false
	unit type (), whose possible value is an empty tuple: ()

	__COMPOUND TYPES__
	arrays like [1, 2, 3]
	tuples like (1, true)
*/

#[allow(unused)]
fn main() {
    // Variables can be type annotated
    let logical: bool = true;
    println!("Logical type: {}", logical);
    
    // Regular vs Suffix annotation
    let a_float: f64 = 1.0; // Regular annotation
    println!("Float type: {:.1}", a_float);
    let an_integer = 5i32; // Suffix annotation
    println!("Integer type: {}", an_integer);

    // Or a default will be used
    let default_float = 3.0; //f64
    println!("Regular Float: {:.2}", default_float);
    let default_integer = 7; //i32
    println!("Regular Integer: {}", default_integer);

    // A type can also be inferred from context
    let mut inferred_type = 12; //Type i64 is inferred from another
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; //Mutable i32
    mutable = 21;

    // Error! The type of a variable can't be changed.
    //mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
}

/* Implementing fmt::Display for a structure where the elements must each be handled sequentially is tricky. 
The problem is that each write! generates a fmt::Result. Proper handling of this requires dealing with all 
the results. Rust provides the ? operator for exactly this purpose. */


//#[allow(dead_code)] // Deactivate Warning about unused code (FOLLOWING)
//#![allow(dead_code)] // Deactivate Warning about unused code (ALL)
use std::fmt;


// Define a structure named "List" containing a "Vec"
struct List(Vec<i32>);

impl fmt::Display for List {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Extract the value using tuple indexing,
		// and create a reference to `vec`.
		let vec = &self.0;

		// 
		write!(f, "[")?;


		// Iterate over "v" in "vec" while enumerating the iteration
		// count in "count".
		for (count, v) in vec.iter().enumerate() {
			// For every element except the first, add a comma.
			// Use the ? operator, or try!, to return on errors.
			if count != 0 { write!(f, ", ")?; }
			// EXERCISE: Try changing the program so that the index of each element in the vector is also printed.
			write!(f, "{}: {}", count, v)?;
		}

		//Close the opened bracket and return a fmt::Result value.
		write!(f, "]")
	}
}



fn main() {
    let v = List(vec![1, 2, 3, 4, 5, 6]);
    println!("{}", v);
}

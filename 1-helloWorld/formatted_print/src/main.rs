use std::f64::consts::PI; //required by exercise

fn main() {
    
    //without suffix 31 become i32
    println!("{} days.", 31);

    //positional arguments
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    //named arguments
    println!("{subject} {verb} {object}", 
    	object="the lazy dog", 
    	subject="The quick brown fox", 
    	verb="jumps over");

    //special formatting
    println!("{} of {:b} people know binary, the other half doesn't.", 1, 2);

    //right-align text
    println!("{number:>width$}", number=1, width=6);


    //EXERCISE: Print "Pi is roughly 3.142" by controlling the number of decimal places shown.
    println!("Pi is roughly {:.3}", PI);

    //EXERCISE: Fix broken code
    /*#[allow(dead_code)]
    struct Structure(i32);
    // FIXME: Custome structure require more complicated handling to print.
    println!("This struct {} won't print...", Structure(3));*/

    #[allow(dead_code)] //this attribute disables the lint that will warn about unused code
    #[derive(Debug)] //now you can print with {:?}
    struct Structure(i32);
    println!("This struct {:?} will now print...", Structure(3));


}

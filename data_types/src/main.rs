// DATA-TYPES

use std::io;
fn main() {
    // Rust is STATICALLY type Language: Meaning it knows all type of variable at compile time
    // Every value in Rust is a certain data type
    // : u32 is the default type annotation for integers 
    
    // println!("Hello, world!");
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("the value of guess is: {guess}")

    // THERE ARE TWO TYPES OF DATA-TYPES IN RUST
    // *Scalar type:  (integers, booleans, float-points, numbers and characters)
    // *Compound type:

    // INTEGER 
        // Integer is a number without a fraction component
        // u32 is one integers type
        // u32 simply means that the value associated with should be an unsigned integer
        // And takes up a 32bit memory space
        // EXAMPLE
            // let guess: u32 = "42".parse().expect("Not a number!");


    // FLOATING - POINT
    // Rust has two primitive types for floating-numbers. Which are numbers with decial points.
    // Rust floating-point types are f64 for 64bit and f32 for 32bit
    // The default type is f64 bcos modern CPU's speeds are roughly the same as f32
    // Floating point numbers are represented according to IEEE-754 standard.
    // f32 is single precision float while f64 is double precision float.

    // NUMERIC OPERATIONS
    // Rust support the basic mathematical operations. Eg.
    // *Addition +
    // *Subtraction -
    // * Multiplication *
    // *Division /
    // *Remainder %

    // BOOLEAN TYPE
    // Boolean types have two possible value...... True or False
    // Boolean are one byte in size.
    // Boolean types are specifically using bool
    // EXAMPLE
    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // CHARACTER TYPE
    // It is specific with char using a single quote ('')
    // Rust char type is four bytes in size.
    // It represent a Unicoin scalar which means it can represent a lot more 
    // than a ASCII
    // EXAMPLE
        // fn main() {
        //     let c = 'z';
        //     let z: char = 'ℤ'; // with explicit type annotation
        //     let heart_eyed_cat = '😻';
        // }

    // COMPOUND TYPES
    // It can multiples values into one type.
    // Rust has two primitive compound type.... tuples and arrarys.
    
    // THE TUPLES TYPE
    // It is a way of grouping together number of values with variety types into 
    // one compound type
    // Tuples have fix length. Once declear, it cannot be shrink in size.
    // We can also access tuple element using (.) by add dot plus the index of the value
    // EXAMPLE
        // fn main() {
        //     let tup = (500, 6.4, 1);
        
        //     let (x, y, z) = tup;
        
        //     println!("The value of y is: {y}");
        // }
        // OR
        // fn main() {
        //     let x: (i32, f64, u8) = (500, 6.4, 1);
        
        //     let five_hundred = x.0;
        
        //     let six_point_four = x.1;
        
        //     let one = x.2;
        // }
    
// THE ARRARY TYPE
    // Every element of an arrary must have the same type
    // Arrary in Rust has a fixed length
    // We write the values in an array as a comma-separated list inside square brackets:
    // EXAMPLE
        // fn main() {
        //     let a = [1, 2, 3, 4, 5];
        // }
        // let todo = ["integers", "boolean", "float-points", "numbers", "characters"];
        // let days = ["D"; 4];
        // let monday = todo[0];
        // let tuesday = todo[1];
        // let wednesday = todo[2];
        // let thursday = todo[3];
        // let friday = todo[4];
        //     println!("days: {}", days[0]);
        //     println!("I have been learning {monday}, {tuesday}, {wednesday}, {thursday}, {friday} !");


            let a = [1, 2, 3, 4, 5];
        
            println!("Please enter an array index.");
        
            let mut index = String::new();
        
            io::stdin()
                .read_line(&mut index)
                .expect("Failed to read line");
        
            let index: usize = index
                .trim()
                .parse()
                .expect("Index entered was not a number");
        
            let element = a[index];
        
            println!("The value of the element at index {index} is: {element}");
        


}


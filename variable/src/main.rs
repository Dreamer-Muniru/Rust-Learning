// VARIABLE & MUTABILITY

// Variables are immutable by default
// But we can make it mutable by adding 'mut' in front of the variable name
fn main() {
    let date = 2022;
    println!("We're on... {date}");



// CONSTANT
    // Constants values are bound to a name which are not allowed to change
    // You can't use mut with contant like we did with variable
    // Constant are always immutable
    // We declear constant using the const keywords
    // Constant can be declear in any scope including global scope
    // The naming conversion must be all UPPER CASE with underscores between words
    // EXAMPLE
    // const DATE_OF_BIRTH: u32 = 25;

// SHADOWING
    // Shadowing you can declear a new variable with the same name as the previous
    // The second name is what the compiler will see when you use the name of the variable
    // It allows you to re-assign the variable name to a new value
    // Rust allowed you to use the 'let' keyword for declearation
    // EXAMPLE
      
        let z = 30;
        let z = z + 20;
        println!("The value of z is: {z}")



}


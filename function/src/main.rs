// FUNCTION
    // fn keyword is used to declear a new function
    // Rust uses snake case for both function and varible names.
    // All letters are lowercases and underscores separate words.
    // We can call any function defined by entering its name followed by parentheses
    // The curly brackets tells compiler with function body begins and end.

// fn main(){
//     print!("Hello World");

//     second_function();
// }

// // Another function
// fn second_function(){
//     println!("This is the Second function");
// }

// Function can also take in parameters
// fn main(){
//     para_function(5);
// }

// fn para_function(x: i32){
//     println!("The value of x {x}");
// }

// STATEMENTS AND EXPRESSIONS

// STATEMENT
// Statements are instructions that perform some action and do not return a value
// let x = 5 is example of a statement
// Statements does not return a value


// EXPRESSIONS
// Expressions evaluate to a value, they return that value
// Rust is an expression base language
// Expression can be part of statement
// eg. let y = 6;
// Calling a function is an expression
// Expressing does not include semi-colon at the end else it will turn into a statement
// EXAMPLE
    // {
    //     let x = 3;
    //     x + 1
    // }
    // fn main() {
    //     let x = plus_one(5);
    
    //     println!("The value of x is: {x}");
    // }
    
    // fn plus_one(x: i32) -> i32 {
    //     x + 1
    // }
    

fn main(){
    let x = plus_two(7);
    print!("The value of x is {x} ")
}

// Function with return value
// We don't name return value. We must declear their type after them using a arrow (->) 
fn plus_two(x: i32) -> i32{
    x + 4
}


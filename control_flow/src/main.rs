// CONTROL FLOW
// An if expression allows you to branch your code depending on condition
// // EXAMPLE
// fn main() {
//     let number = 25;
//     if number > 12 {
//         print!("Number is greater 12")
//     }else{
//         println!("Hello, world!");
//     }
// }


// fn main(){
//     let number = 11;
//     if number % 4 == 0{
//         println!("Number is divisible by 4");
//     }else if number % 3 == 0 {
//         println!("Number is divisible by 3");
//     }else if number % 2 == 0 {
//         println!("Number is divisbile by 2");
//     }else{
//         println!("Number is not divisible by 4, 3 and 2");
//     }
// }

// All 'if' expressions with the keywords if follow by condition
// Block of code associated with if expression are sometimes called arms.

// fn main() {
//     let condition = true;
//     let number = if condition { 6 } else { 5 };

//     println!("The value of number is: {number}");
// }


// REPEATIION IN LOOP
// Loop keyword tell rust to run a block of code over and over again until you tell it to stop
// Example
    // fn main () {
    //     loop{
    //         println!("try again");
    //     }
    // }
// 'Continue in loop tells the program to skip over any remaining code 
// in this iteration of the loop and go to the next iteration.
// RETURNING VALUES FROM A LOOP

fn main() {
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter + 30
        }
    };
    println!("The value of result is {result} ")
}

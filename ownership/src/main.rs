//                 // OWNERSHIP
// // Ownership is a set of rules that govern how Rust program Manage its Memory.
// // Basically, every program in Rust are supposed to manage the way they use 
// // the computer's memory while running.

//     // RUST USES A THIRD APPROACH
// // Unlike languages like JAVA and C# that uses Garbage Collector that automatically 
// // check-ups and clean no-longer use memory to provide space for more programs to run.
//     // RUST programs are manage through a system of ownership which comprises rules
//     // and regulations. If any of the rules are violated the program won't run.

// // STACK AND HEAP
//     //The Computer's Memory is being divided into two part during a runtime of your code.
//     // And this is known as Stack and Heap.

// // STACK
//     // Stack stores values in order it gets them and delete in the opposite order
//     // Stack is the default memory location of your program but unfornately it's limited in size.
//     // In Stack, ADDING Data is known as 'pushing' onto the Stack while DELETING is 'popping' onto the Stack
//     // Pushing data onto the Stack is faster bcos the allocator don't have to search for a place
//     // to store the new data. Since the default location is always at the top of the Stack


// // HEAP
//     // Heap on the other hand is the large area of memory space available for use by a
//     // program in a runtime.
//     // All the data in Heap are less organize and this is making it difficult to allocate data
//     // since you'll have to follow a pointer(address) in order to get to the Data.

// // RULES IN OWNERSHIP
//     // * Each value in a variable has a owner 
//     // * There can ONLY be one Owner at a time
//     // * The value is always dropped anytime the Owner goes out of Scope.
//         // EXAMPLE
//             fn main() {

//                 let name = "Brown";       //variable 'name' is valid and is in Scope
//                 // We can do other things with the above variable here. Eg. println!

//             }    //Over there, the Variable is out of Scope and not Valid anymore
                
// // STRING TYPE
//     // Unlike the data-types we looked at in Chapter 3.2, to fully understand the concept of 
//     // Ownership, we need another data-type that is more advance. Even though String Literal
//     // could be cool but as there're known in size and their values can drop off when they're
//     // out of Scope. Therefore they're suitable for Stack.

//     // We actually want a data-type that can store on HEAP and search through how Rust knows
//     // when to clean up data, when it is not longer in use.
//     // With This, STRING TYPE is the great option we'll be using from hence forth

// // STRING TYPE is manages data allocated on the HEAP and as such is able to store an amount of 
// // text that is unknown to as at compile time. And this could be done using 'from()' function
//     // EXAMPLE
//         // We could create a String from String literal using the below 'from()' function 
//         // as shown below:
//          let s = String::from("Hello");

// // MEMORY ALLOCATION
//     // Right now, in order for the String type to support a mutable, growable piece of text,
//     // we need to allocate a certain amount of space on the HEAP part of the memory unknown at a compile time,
//     // to hold the content.
//     // WHAT THIS MEANS IS THAT:
//         // The memory must request a space from the Memory Allocator at runtime
//         // And we also need to find a way to return to this memory to the Allocator 
//         // after we're done with our string
//     // THERE ARE TWO WAYS THESE COULD BE DONE
//         // FIRST
//             // The first part is done by us when we call 'String::from()' function.
//             // And this implementation request the memory it needs.
//         // SECOND
//             // Since Rust is not a Garbage Collector language, it is ours duty to identify
//             // memory that is no longer being used and call code to explicitly free it.
//     // RUST APPROACH IN MEMORY ALLOCATION
//         // Rust takes different approach by allowing memory to automatically return once 
//         // the vairable that owns it goes of scope.
//         // EXAMPLE
//            {
//                let s = String::from("Hello")   //s is valide from this point forward.
//             //    Do other things with the variable s here.
//            } //Scope is now over,and s is no longer valid
//         // Rust has a special function call 'drop' which allows the owner of the string 
//         // to return the memory. Drop is call automatically after the closing curly bracket

//         // LIMITATION
//             // The process may have a clear impact on the way Rust code is written. It may seems
//             // simple right now. But behavior of our code can be unexpected in a situation where
//             // multiples variable are trying to use the same data we've allocated on the HEAP.
    
//     // HOW VARIABLE AND DATA INTERACT
//         // EXAMPLE USING THE INTEGERS VERSION
//          let x = 5;     //We declear a variable x here
//          let y = x;     //Then I make a copy of the value in x and bind it to y
//                         // Now y has the value of x which is equal to 5

//         // EXAMPLE USING THE STRING VERSION
//         fn main(){
//             let s1 = String::from("Hello");
//             let s2 = s1;
//             println!("{}", s1)      //Code with Error 
//         }
//         //CAUSE OF ERROR
//             // When we assign s1 to s2, the string data such as Pointer, Length and Capacity
//             // is copied but does not copy the data on the HEAP that the pointer refers to.
//         // LENGTH: Is the amount of memory in bytes, the content of the String is currently using
//         // CAPACITY: Is the totol amount of memory in bytes the String has received from the Allocator

//         // WAYS VARIABLE AND DATA INTERACT: CLONE
//             // In an instance where we want to deeply copy the Heap Data of a String, not necessarly 
//             // the Stack data, we can use a method called 'clon' to achieve that.
//             // EXAMPLE OF CLONE METHOD
//                 let s1 = String::from("Hello");
//                 let s2 = s1.clone();
//                 println!("s1 ={}, s2={}", s1, s2)
// // Ways Memory Interact with Data: Clone
// fn main(){
//     let s1 = String:: from("Hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 ={}", s1, s2)
// }
// // This code works explicitly find

// // STACK-ONLY DATA: COPY
//     // Another wrinkle example on Integers
//     let x = 5;
//     let y = x;

//     println!("x = {}, y={}", x, y)

//     // The above code works fine and it also contradict everything we learn
//     // about clone method. As x is still valid and has not been move to y.

//     // REASON
//         // The reason is that, data-type such as integers that have known size at
//         // compile time are stored entirely on the Stack. So copies of the actual
//         // values are quick to make. This means there's no reason to prevent x from
//         // being valid after we created the variable y. So calling CLONE wouldn't do
//         // any different than leaving it.

//     // THE COPY ANNOTATION
//         // Copy trait is a Rust special annotation that allows enable us to place on types
//         // that are stored on the Stack as integers are.
//         // So if a type implement the copy trait, variables that use it do not move, but 
//         // rather are trivially copied and this makes them still valid after we assign it
//         // to another variable.
        
//     // EXAMPLE OF DATA - TYPE THAT IMPLEMENT: COPY
//         // * All integers type, such as u32
//         // * The Boolean types, bool with values of true or false
//         // * All floating point type such as f64
//         // * The character type or char type
//         // * The Tuples if they onlyl contain types that implement copy. eg. (i32, i32)
//         //   implement copy but (i32, string) does not implement copy.


//     // OWNERSHIP FUNCTION
//         // The method we used in passing values to variable is same that goes for Function
//         // Passing values to function will move or copy just like the assigment does.

//         // EXAMPLE
        fn main() {
            let s = String::from("hello");  // s comes into scope
        
            takes_ownership(s);             // s's value moves into the function...
                                            // ... and so is no longer valid here
        
            let x = 5;                      // x comes into scope
        
            makes_copy(x);                  // x would move into the function,
                                            // but i32 is Copy, so it's okay to still
                                            // use x afterward
        
        } // Here, x goes out of scope, then s. But because s's value was moved, nothing
          // special happens.
        
        fn takes_ownership(some_string: String) { // some_string comes into scope
            println!("{}", some_string);
        } // Here, some_string goes out of scope and `drop` is called. The backing
          // memory is freed.
        
        fn makes_copy(some_integer: i32) { // some_integer comes into scope
            println!("{}", some_integer);
        } // Here, some_integer goes out of scope. Nothing special happens.
        
// RETURN VALUES AND SCOPE
    // Returning values in Rust can also transfer it's ownership

    // EXAMPLE OF FUNCTION THAT RETURN OWNERSHIP

    fn main() {
        let s1 = gives_ownership();  // gives_ownership moves its return value into s1
    
        let s2 = String::from("hello");     // s2 comes into scope
    
        let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also
                                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.
    
    fn gives_ownership() -> String {    // gives_ownership will move its
                                // return value into the function that calls it
    
        let some_string = String::from("yours"); // some_string comes into scope
    
        some_string          // some_string is returned and moves out to the calling function
    }
    
    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                          // scope
    
        a_string  // a_string is returned and moves out to the calling function
    }
    


// RETURNING OWNERSHIP OF PARAMETERS
    // Taking ownership and returning ownership with every function is a kind of slow journey.
    // Especially if we want to let a function use value without taking Ownership. 
    // This could be quite boring that everything we pass in needs to be pass out if we want to
    // use them.


    // RETURNING MULTIPLE VALUES USING TUPLE
        // The good thing is that Rust allows us to return multiple variable using tuple 
        // as shown in the code below.
    
        fn main() {
            let s1 = String::from("hello");
        
            let (s2, len) = calculate_length(s1)
        
            println!("The length of '{}' is {}.", s2, len);
        }
        
        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() returns the length of a String
        
            (s, length)
        }

        // This code will print:...... The length of 'hello' is 5.
        

        // ==================================================
                // REFERENCES AND BORROWING
        // ==================================================

// One issue with the above code is that we have to return the String to the calling function
// so that we can still use the String after the call to calculate_length() all because the
// String was being moved into the calculate_length().
    // ALTERNATIVELY
        // Instead we can also provide a reference to the String value.
    // REFERENCE
    // A Reference is like a pointer, in that it's an address we can follow to access a data
    // store at that address. That data is owned by some other variable.

    // Unlike a pointer we learned on Heap, A reference is a guaranteed to point to a valid 
    // value of a particular type for the life of that reference.

    // EXAMPLE WITH CODE
        // You can use calculate_length that has a reference to an object as a parameter
        // instead of taking ownership of that value.
        fn main() {
            let s1 = String::from("hello");
        
            let len = calculate_length(&s1);
        
            println!("The length of '{}' is {}.", s1, len);
        }
        
        fn calculate_length(s: &String) -> usize {
            s.len()
        }

        // HOW THE ABOVE CODE WORKS
            // From the above code, we have realised that all the  tuple code in a variable 
            // decleration and function return values are gone. And inside the fuction,
            //  we pass &s1 into the calculate_length which create a reference of the value
            // s1 but does not owns it, and the value will drop when the reference stopped
            // being used.

        // MORE EXAMPLE
        fn calculate_length(s: &String) -> usize { // s is a reference to a String
            s.len()
        } 

        // From the above function, s is a reference to a String.
        // After the closing curly brackets, s goes out of scope. And because it does not
        // have ownership of what it refers to, it is not dropped.
        
        // NOTE: When a function have reference as paramaters instead of the actual value,
            // we won't need to return the values in order to get ownership because we never
            // had ownership.

        // Borrowing.
        // The action of creating a reference is known as borrowing.
        // CAN WE MODIFY SOMETHING BEING BORROWED?
            // EXAMPLE
            fn main() {
                let s = String::from("hello");
            
                change(&s);
            }
                                            // CODE WITH ERROR (ATTEMPTING TO MODIFY A BORROW VALUE)
            fn change(some_string: &String) {
                some_string.push_str(", world");
            }
            //Just as variable are immutable by default. We are not allowed to modify something
            // we have a reference to.
        
        // MUTABLE REFERENCE
            // We can modify a borrowed values using 'mutable reference'
            // Lets retweat the above code with error to modify the borrowed value.
            fn main(){
                let s String::from("hello");
                change(&mut s);
            }
            fn change(some_string: &mut String){
                some_string.push_str(", world");
            }

            // 





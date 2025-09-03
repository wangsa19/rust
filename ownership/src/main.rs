// variable scope
// fn main() {
//     {
//         // s is not valid here, since it's not yet declared
//         let s = "hello"; // s is valid from this point forward

//         // do stuff with s
//         println!("{s}");
//     } // this scope is now over, and s is no longer valid

//     // println!("{s}");
// }

// the stringg type
// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{s}"); // this will print `hello, world!`
// }

// Memory and Alloction
// fn main() {
//     {
//         let s = String::from("hello"); // s is valid from this point forward

//         // do stuff with s
//     } // this scope is now over, and s is no
//     // longer valid
// }

// Variables and Data Interacting with Move
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}, world!");
// }

// Scope and Assignment
// fn main() {
//     let mut s = String::from("hello");
//     s = String::from("ahoy");

//     println!("{s}, world!");
// }

// Variables and Data Interacting with Clone
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
// }

// Stack-Only Data: Copy
// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x = {x}, y = {y}");
// }

// Ownership and Functions

// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // Because i32 implements the Copy trait,
//                                     // x does NOT move into the function,
//                                     // so it's okay to use x afterward.

// } // Here, x goes out of scope, then s. However, because s's value was moved,
//   // nothing special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.


// Return Values and Scope

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
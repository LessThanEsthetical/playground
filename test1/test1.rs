use std::io;

// By default in Rust, you don't need to write string with \n at the end
// The macro called println!() does that automatically
// However, if you need to print without line break, use print!() macro (it supports \n as well)

// Unlike C, variables are always created (with let) as immutable and don't need definition
// (int, float, char etc.), but can be specified as such (let luck: i32 = 19;) yet required for const
// variables (let const luck: f64 = 19.19;)
// Difference between const and immutable variable is that immutable variable can be changed with same
// variable with new value introduced (old one will be discarded), unlike const type
// Then how to change value itself? Declare variable as mutable (using mut: let mut age: i32 = 20;)

// Also, Rust does not require return 0; at the end of function

// To compile this file, install Rust toolchain and use command "rustc ./test1.rs", you'll get then test1 binary in same directory

//fn main() {
//
//    // There's no implemented scanf() of C nor input() of Python, so going little verbose here
//    println!("Enter two numbers to compare separated by whitespace ('XX YY'): ");
//    let mut input = String::new();
//    io::stdin().read_line(&mut input).expect("Error: failed to read the line.");
//    let num: Vec<&str> = input.split_whitespace().collect();
//    // We get a and b variables defined here. Compiler doesn't allow to initialize early empty ones
//    let a: i32 = num[0].parse().expect("Invalid first number.");
//    let b: i32 = num[1].parse().expect("Invalid second number.");
//
//    let c = { a < b };
//
//    // Instead of %d we use {} as placeholder here, reference is the same
//    println!("c = {}.", c);
//
//    if c {
//        println!("First number is less than second.");
//    }
//    else {
//        println!("Second number is less than first or equal.");
//    }
//}

//fn main() {
//
//    // Same as in C, variables can reference others' variable values
//    let is_locked = true;
//    let can_edit = !is_locked;
//
//    println!("is_locked = {}.", is_locked);
//    println!("can_edit = {}.", can_edit);
//}

//fn main() {
//
//    // Unlike C, these will be casted as bool than int
//    let a = { -2 > 5 };
//    let b = { 0 < 7 };
//    let c = { 0 == 0 };
//    let d = { a && b || c };
//
//    // So you need to cast them there as integers
//    println!("a = {}.", a as i8);
//    println!("b = {}.", b as i8);
//    println!("c = {}.", c as i8);
//    println!("d = {}.", d as i8);
//}

//fn main() {
//    const a: i32 = 19;
//    const b: i32 = 10;
//    let c = { a < b };
//
//    println!("c = {}.", c);
//
//    // Rust and Go doesn't have ternary operators like those of C and Python AFAIK
//    // It's still possible to make one line condition albeit it'll be highly unreadable
//    if a < b { println!("First number is less than second."); } else { println!("First number is bigger than or equal to second."); }
//}

//fn main() {
//
//    println!("1 for hello\n2 for bye\nChoose number: ");
//    let mut input = String::new();
//    io::stdin().read_line(&mut input).expect("Failed to read the line.");
//    let choice: i32 = input.trim().parse().expect("Input is not an integer.");
//
//    // There's no switch case in Rust, instead I'm going to use match
//    match &choice {
//        1 => println!("Hello!"),
//        2 => println!("Bye!"),
//        _ => println!("Why you didn't provide 1 or 2? :("), // Underscore works as "default" case
//    }
//}

//fn main() {
//    // In Rust formatting style, it is adviced to use UPPERCASE for variable names
//    // The exclamation mark "!" is Rust equivalent to C tilde "~" for bitwise operators
//    // Also, there's no "mods" for variable types (like "unsigned" for char)
//    // But since unsigned char equals 255 bytes, we can assume unsigned integer (u8) will do same trick
//    const A: u8 = 153;
//    const B: u8 = !A;
//
//    // Take 8, the number of available bits, add 2 (because of 0b at the start), then put that
//    // number between # and b, so it becomes {:#010b}
//    println!("A = {}, binary = {:#010b}", A, A);
//    println!("B = {}, binary = {:#010b}", B, B);
//}


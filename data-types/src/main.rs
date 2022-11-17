fn main() {
    println!("Hello, world!");
    // EX 1:
    let guess: u32 = "42".parse().expect("Not a number!"); // we need u32 number type, otherwise rust compiler won't know what number type to use
    println!("Value of guess: {guess}")
}

// Rust is statically typed. We don't always have
// to declare variable types when declaring variables
// with let, but we do have to do so when we're
// declaring a variable where many different types are
// possible. See EX 1 in main.

// SCALAR TYPES

// In Rust, scalar types represent a single value.

// There are four primary scalar types in Rust:
// integers, floating-point numbers, Booleans, and characters.

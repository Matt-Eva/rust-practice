fn main() {
    println!("Hello, world!");
    another_function();
    func_with_params(5);
    func_with_multi_params(5, 'i');

    let y = 10;

    let z = { // <-- we've defined a new scope block with these curly brackets to create a new expression
        let x = 4;
        x + 2
    };
    println!("y is: {y}. z is: {z}.");
    
    let n = five();
    let m = six();
    println!("n is : {n}. m is: {m}")
}

fn another_function() {
    println!("Another function.");
}

fn func_with_params(x: i32) {
    println!("The value of x is: {x}");
}

fn func_with_multi_params(y: i32, label: char){
    println!("The measurement is: {y}{label}")
}

// STATEMENTS AND EXPRESSIONS

// Rust has some cool expression features.

// Statements do not have any sort
// of return value, but expressions do:

// let y = 10; // <-- statement - has no return value.

// let z = { // <-- we've defined a new scope block with these curly brackets to create a new expression
//     let x = 4;
//     x + 2
// };

// Note that expressions do not include 
// ending semicolons.

// RETURN VALUES   

// Functions in Rust use implicit return,
// similar to Ruby. You can also use explicit
// return to exit a function early.

// If a function has a return value, its
// return type must be declared. It is declared
// following the -> notation in the function
// declaration.

fn five() -> i32 {
    5
}
// this function uses explicit return to 
// return 5.

fn six() -> i32 {
    return 6
    // println!("5"); // <-- unreachable code - triggers compiler warning.
}
// This function uses explicit return
// to return 6. The return keyword causes the
// function to exit.
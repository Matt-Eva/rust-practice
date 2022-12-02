fn main() {
    let _s = "hello"; // string literal declaration immutable.

    let mut st = String::from("hello"); // declared using string type makes this string mutable.

    st.push_str(", world!");

    println!("{}", st);

    let x = 5;
    let mut y = x; // value of y is a COPY of x

    y += 1;

    println!("{y}");
    println!("{x}");

    let s1 = String::from("hello");
    let s2 = s1; // value of s2 points to the same
                 // location in heap memory as s1.
                 // For this reason, s1 is considered no longer
                 // valid after s2 is declared. Basically, s1
                 // has MOVED to s2 (that's how Rust refers
                 // to it).
                 // println!("s1: {s1}"); // <-- throws error, since s1 is now invalid
    println!("s2: {s2}.");
    // automatic copying never results in a deep
    // copy, unless it's copied on the stack, in
    // which case it defaults to a deep copy.

    // DEEP COPYING - CLONE

    let s3 = String::from("Hi");
    let mut s4 = s3.clone(); // we can use the clone method
                             // to make a deep copy of a variable on the heap. I.e.
                             // new memory is allocated on the heap and a
                             // new pointer is created.

    s4.push_str(" there!");

    println!("s3 = {}, s4 = {}", s3, s4);

    // Rust has a special annotation called the Copy trait
    // that we can place on types stored on the stack.
    // (More on traits later).
    // If a type implements a copy trait, variables are not
    // moved but rather *trivially copied* and are still
    // valid after assignment to another variable.

    // OWNERSHIP AND FUNCTIONS

    // Some truly interesting behavior here -
    // when you have a variable that does not implement
    // the copy trait - like a string - when you
    // pass that variable as an argument to a function,
    // a move occurs, because Rust is pass-by-value.

    let s = String::from("Hello!");

    takes_ownership(s);
    // println!("{s}");//<-- s is no longer valid, because
    // it has been moved to the parameter for
    // takes_ownership.

    let n = 5;
    makes_copy(n);
    println!("{n}");
    // because n is of type int and is therefore of
    // known size at compile time, it is allocated
    // to the stack and is automatically deeply copied.
    // This means n is still valid even after having
    // been copied to the parameter for makes_copy.

    // RETURN VALUES AND SCOPE

    // Function return values are themselves moves, and
    // can be assigne to other variables.
    // For example:

    let _s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);
    // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    // MULTIPLE RETURN VALUES

    // functions are able to return multiple values
    // in a tuple. We can also declare variables in
    // a tuple to capture those return values (similar
    // to destructuring in JavaScript).

    let (greeting, num) = multiple_returns();

    println!("greeting {}, num {}", greeting, num);

    // Rust does also have a feature that allows
    // us to pass variables to functions without 
    // transferring ownership. This feature is called
    // "references."

    // REFERENCES AND BORROWING
}

fn takes_ownership(some_string: String) {
    println!("Ownership take of {some_string}");
}

fn makes_copy(some_int: i32) {
    println!("Just a copy of {some_int}");
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn multiple_returns() -> (String, i32){
    (String::from("hello"), 6)
}

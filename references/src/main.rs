fn main() {
    // REFERENCES AND BORROWING
    // Rust's refusal to deep copy variables that aren't allocated to the stack
    // - which includes passing them as arguments to functions - is great for
    // memory management, but it's not so good for us as programmers. However,
    // Rust gives us a great tool that allows us to preserve this memory safety
    // while making our lives as programmers easier; references.

    let s1 = String::from("hello");
    let s2 = &s1; // when we reference s2, it will still give us the value of s1.
    let s3 = &s2; // this is also allowed - creating a reference to a reference.
    let s4 = s2;
    println!("{s2}");
    println!("{s3}");
    println!("{s4}");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // we use the & to denote that we are passing a reference instead of
    // the variable itself. A reference is basically a pointer, except in Rust
    // a reference can only point to a valid value of a particular type for the life
    // of that reference.

    // Basically, a reference allows us to refer to a specific value
    // without actually taking ownership of it.

    // The opposite of referencing is "dereferencing", as is accomplished
    // using the dereference operator, *.

    // BORROWING

    // In Rust, creating a reference is referred to as "borrowing".
    // One thing to keep in mind - we can't modify something that we're borrowing

    // change(&s1); <-- references are immutable by default - we can't use this
    // function to change our borrowed value.

    // However, we can create mutable references, that WILL allow us
    // to change borrowed values:

    let mut st = String::from("hello");

    change(&mut st);

    println!("{st}");

    // because we declared a mutable variable, set up a function
    // that receives a mutable reference, and passed in that mutable
    // reference as the argument, we can now modify our original variable
    // via a reference.

    // NOTE: creating a mutable reference to a value inhibits you from having
    // any other references to that value:

    let r1 = &mut st;
    // let r2 = &mut st; // <-- causes error
    // println!("{}, {}", r1, r2);
    println!("{r1}");

    // note that this isn't because our original variable is mutable - rather
    // that because the reference we created was mutable.

    let mut a = String::from("hello");

    let r2 = &a;
    let r3 = &a;
    // let r6 = &mut a; // <-- causes error
    println!("{}, {}", r2, r3);
    a = a + " world";
    println!("{a}");

    // This feature - only one mutable reference at a time - is what
    // Rust uses to prevent data races, which can happen when the following
    // three behaviors occur:

    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.

    // Correction - we can have multiple mutable references. We just can't
    // have SIMULTANEOUS mutable references. For example, this is allowed:

    {
        let r4 = &mut a;
        println!("{r4}");
    }

    let  r5 = &mut a;
    println!("{r5}");

    // This holds true for ANY reference - can't have simultaneous mutable and immutable
    // references to the same value. (See example above.)

    // There is a slight exception: so long as an immutable reference isn't
    // used after the creation of a mutable reference, everything is ok:

    let mut b = String::from("hello");

    let r6 = &b;
    let r7 = &b;
    println!("{}, {}", r6, r7);

    let r8 = &mut b;
    println!("{r8}");

    let r9 = &b;
    // r8.push_str("world"); // <-- This would cause an error, since we're using our mutable reference after creating another immutable reference
    println!("{r9}");


    // DANGLING REFERENCES
    // In languages that use pointers, programmers can accidentally create
    // something known as a "dangling pointer" - a pointer which points to
    // a location in memory that has actually been freed. (The original value
    // that the pointer pointed to has been freed from memory).

    // Rust doesn't allow dangling references - if there is a reference to 
    // some data, the Rust compiler makes sure that the data will not go
    // out of scope before the reference to the data does:

    let _reference_to_nothing = dangle();

}

fn calculate_length(s: &String) -> usize {
    // declaring parameter type to be a reference to a String type variable
    receives_reference(s);
    s.len()
} // s, the parameter for this function, now goes out of scope.
  // However, since it doesn't own the value we wanted it to reference
  // we can still use that value in an outer scope.

fn receives_reference(s: &String) {
    println!("{s}");
}

// fn change(some_string: &String){
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    // &s // this will cause an error, since we're trying to return a reference to
    // a value that will go out of scope. Just return the value instead:
    s
}

fn main() {
    let mut x = 5; // Without mut, this variable would be immutable
    println!("The value of x is: {x}");
    x = 6; // x is still statically typed, though we did not declare its type
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // variable shadowing

    let y = 5;

    let y = y + 1; // this shadows previous y

    {
        let y = y * 2; // this shadows both previous y's but only for the duration of this scope
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // datatype change with variable shadowing

    let spaces = "    "; // string datatype
    let spaces = spaces.len(); // number datatype
    println!("{spaces}")
}

// Variables always start off as immutable
// If we want a variable to be mutable
// we have to append the mut keyword before the
// variable declaration, as seen above.

// Only variables declared with let can
// be modified to be mutable.

// Declaring a variable with const will
// preserve its immutability - it can't be
// modified with mut.

// const also requires that we declare the
// variable datatype. Note that let does not
// - however, variables declared with let still are
// statically typed. You cannot assign your variable
// a different datatype even if you use the mut keyword.

// For reference on how to reassign a variables datatype
// and value, see the shadowing section.

// the naming convetion in rust dictates
// that constants should be all uppercase and should
// use underscores between words.

// VARIABLE SHADOWING (similar to Go)

// We can shadow a variable by essentially redeclaring
// it with the let keyword. The compiler will see
// the second variable declared when using the name
// of that variable. So the second variable "overshadows"
// the first, until either it is shadowed or the scope ends.

// One of the benefits of shadowing is that it allows
// us to actually reassign the datatype of the variable
// being shadowed.
fn main() {
// SLICE TYPE
// In Rust, a "slice" lets you reference a contiguous
// sequence of elements in a collection rather than
// the whole collection. Slices are a type of reference,
// so they do not have ownership.

let mut s = String::from("hello world");

let _word = find_first_word(&s); // word get's the value of 5 using our function

s.clear(); // this empties s, setting it to ""

// word still has the value 5 here, but there's no more string
// that we could use 5 with, which means that word is now useless.

// This is a tedious problem to have to deal with - moreover, if we
// wanted to write a "second word" function, we'd need to return both the starting and ending index, which would rely on data in a particular state
// that aren't actually tied to that state. All three variables would be unrelated but would need to be kept in sync - this is a pain.

// STRING SLICES

// Rust offers a convenient solution to this problem - string slices.

// we can generate string slices using index values in a range and references:

let mut m = String::from("hello world");

let hello = &m[0..5]; // &m[..5] is also valid, since it's starting at 0
let world = &m[6..11]; // &m[6..] is also valid, since the final value is the ending index of the string.

println!("{}, {}", hello, world);

let word = first_word_with_slice(&m);

// m.clear(); //<-- This throws an error. Because word is an immutable reference, we cannot create a mutable reference before word falls out of scope. Because clear is needs to truncate the String, it needs to get a mutable reference, which causes the "mutable borrow" error we see in the terminal.

println!("the first word is: {}", word);

m.clear(); // this doesn't throw an error, because we're not trying to reference "word" after this line, which means it's ok to make a mutable reference.

// STRING LITERALS ARE SLICES

// A string literal is actually a slice:

let _y = "hello, world!";

// The type of y is &str - it's a slice pointing to that specific point of the binary. It's an immutable reference - that's why string literals are immutable.

// STRING SLICES AS PARAMETERS

// we can pass string slices directly as parameters

let my_string = String::from("hello world");

// `first_word` works on slices of `String`s, whether partial or whole
let _word = best_first_word(&my_string[0..6]);
let _word = best_first_word(&my_string[..]);
// `best_first_word` also works on references to `String`s, which are equivalent
// to whole slices of `String`s
let _word = best_first_word(&my_string);

let my_string_literal = "hello world";

// `best_first_word` works on slices of string literals, whether partial or whole
let _word = best_first_word(&my_string_literal[0..6]);
let _word = best_first_word(&my_string_literal[..]);

// Because string literals *are* string slices already,
// this works too, without the slice syntax!
let word = best_first_word(my_string_literal);

println!("{word}");

// OTHER SLICES

// String slices are specific to strings, but there is a more general slice type as well:

let a = [1, 2, 3, 4, 5];

// we might want to refer to aprt of this array - can do so by making a slice

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);


}

// build a function that finds the first complete word in a string
// (looking for the first space). If there are no spaces, return the 
// whole string.

// We're passing a reference as our perameter - we don't want ownership,
// so this is good. But what can we return? We don't have a way to 
// talk about a part of a string. But, we can use the index at the end of the
// first word
fn find_first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // this method converts our string into an array of bytes

    for (i, &item) in bytes.iter().enumerate(){ 
        // the iter method allows us to create an iterator over our array of bytes
        // it returns each element in a collection.
        // The enumerate method then wraps the result of iter and returns
        // each element as part of a tuple instead. The first element of the tuple
        // return by enumerate is the index - the second element is a reference
        // to the element.
        if item == b' '{ // this is "byte-literal syntax" - we're looking for the first byte that contains an empty space
            return i;
        }
    }

    s.len() // otherwise, we return the length of the string.
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // this just returns a slice of the whole string.
    // this function also now returns a slinge value that is tied
    // to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

    // Because the compiler ensures that references to a value remain valid, we won't have the same problem as before, where we have unrelated variables we need to keep track of together. 
}


// This is the preferred string signature for this particular funciton:
// It allows us to pass in a string slice, or a reference to a string.
// This flexibility takes advantage of deref coercions, which is a featured covered later.
// Defining our function in this manner makes oure API more general and useful without compromising any functionality.
fn best_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {

    // to create a struct, we define an "instance" of that
    // struct by passing in concrete values for each field:

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);
    // note that we use dot notation to get a specific value from
    // a struct.

    // if we create a mutable instance, we can also change the values
    // of a field using dot notation:

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.sign_in_count += 1;
    println!("{}",user2.sign_in_count);

    //note that we can't mark specific fields as mutable - the
    // whole instance must be mutable.

    let user3 = build_user(String::from("Dave@dave.com"), String::from("DaveDave"));
    println!("{}", user3.username);

    // STRUCT UPDATE SYNTAX:

    // long-from way to create a new user based on an old user;

    // let _user4 = User{
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // creating updated user using update syntax (like spread op):

    let _user5 = User {
        email: String::from("another@example.com"),
        ..user1 // this counts as a move for the String in the
        // username field of user1 (since Strings are stack allocated).
        // This means we can no longer reference user1. 
        // If we hadn't moved the String, then this wouldn't cause a problem.
    };

    // which means this throws a move error...
    // let _user6 = User {
    //     email: String::from("hey youg"),
    //     ..user1
    // };

    // TUPLE STRUCTS

    // We can define structs that look similar to tuples called tuple Struct.
    // They have the added meaning the struct name provides, but don't have any field
    // names - they just have the types for the fields. 

    // These are useful when you want to make a tuple a different type from 
    // other tuples, or when naming a field in a regular struct would be verbose
    // or redundant.

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 12, 3);
    let _origin = Point(1, 4, 2);
    // ^^ Rust recognizes these as two separate types.

    fn takes_color(p: Color){ // can't be passed something of type Point
        println!("{}", p.0)
    }

    takes_color(black);
    // takes_color(origin); // throws error

    // You can still destructure tuple structs into individual pieces
    // and use dot-index notation to access individual values.

    // UNIT-LIKE STRUCTS

    // You can also define structs with no fields! Tehse are called
    // "unit-like structs" because they behave similarly to (), the unit type.

    // These can actually be useful when you need to implement a trait
    // on some type but don't have any data that you want to store in the type
    // itself. (traits will be discussed in more detail later).

    struct AlwaysEqual;
    
    let subject = AlwaysEqual;

    // OWNERSHIP AND STRUCTS - A BRIEF ASIDE:

    // Structs are able to store references to data owned by something else,
    // but we have to use "lifetimes", which is another feature of rust that
    // will be discussed later. Lifetimes ensure that the data referenced by a 
    // struct is valid for as long as the struct is.
    

}

fn build_user (email: String, username: String) -> User {
    User {
        email, // uses field init shorthand, which allows us to declare
               // a field with the key name of the variable and the 
               // value of the variable.
        username,
        active: true,
        sign_in_count: 1,
    }
    // this whole function is return a new instance of the user
    // struct
}

// STRUCTS

// structs are an object-oriented-ish feature of rust - it allows
// us to define certain types that contain data in an organized format.
// They're similar to objects in javascript, but are more like
// object orientation in other languages in that they actually define
// new types that our program can use:

struct User {
    active: bool, // these are called "fields"
    username: String,
    email: String,
    sign_in_count: u64,
}
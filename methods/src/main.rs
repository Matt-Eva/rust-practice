fn main() {
    println!("Hello, world!");

    // Methods are declared similarly to functions, but are defined
    // within the context of a struct, and their first parameter is always
    // "self", which represents the instance of the struct the method is being
    // called on.

    // We're going to update the previous "area" function we built in our 
    // "playing-with-structs" exercise to be a method:
    #[derive(Debug)]
    struct Rectangle {
        width:u32,
        height: u32,
    }

    impl Rectangle { // declaring a method on rectangle. Everything within the impl block will be associated with Rectangle.
        fn area(&self) -> u32{ // &self is short for self: &Self. Within an impl block, Self is an alias for the type that the impl block is for.
            // because we always need to declare self as the first parameter, Rust allows us to just write &self for convenience.
            // Because we're using a reference, this method just borrows Self. Methods can take ownership of self and borrow self mutably as well, just as they can with any other parameter.
            // We will occasionally create a method that takes ownership of self, but such occurrences are rare, and are only used if we want to transform our instance into a new value and prevent our user from using the old value.
            self.width * self.height
        }
        fn print_width(&self, message: &str) { // can write multiple methods in here.
            println!("{}{}",message, self.width);
        }
        fn change_height(&mut self, num: u32){ // note that in order to borrow self mutably, we first have to declare self as a mutable instance. (line 31).
            self.height *= num;
        }
        fn reassign(mut self, width: u32, height: u32) -> Rectangle{
            self.width = width;
            self.height = height;
            self
        }
        fn greater_volume(&self, rect: &Rectangle) -> bool{
            self.area() > rect.area()
         }
    }

    // Rust does something called automatic referencing and dereferencing,
    // which allows you to call a method directly on an instance of your struct
    // even if the method's self param is a reference, a mutable reference, or a
    // pointer. Rust will automatically adds in &, &mut, or * so that the instance matches the signature of the method. 

    let mut rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{}",rect1.area()); // this is how we invoke a method (very similar to javascript)
    rect1.print_width("The width of this rectangle is: ");
    rect1.change_height(10);
    dbg!(&rect1);
    let rect1 = rect1.reassign(130, 13);
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("{}",rect1.greater_volume(&rect2));
    println!("{}",rect1.greater_volume(&rect3));
    

    // ASSOCIATED FUNCTIONS

    // We can declare associated functions within impl blocks that are not methods. (They don't have self as parameters).
    // We invoke these associated functions by writing the struct name along with :: syntax (which is also used for namespacing created by modules, which will be discussed later).
    // Associated functions are often used as constructor functions for structs. Many are called "new", but "new" isn't a special name within rust.

    impl Rectangle{
        fn square(size: u32) -> Self { // associated function. Also a constructor function.
            Self {
                width: size,
                height: size,
            }
        }
        
        fn width(&self) {
            println!("called the method width, which prints the property width: self.width == {}", self.width);
        }
    }

     // As shown above, we can also have multiple impl blocks. There's no particular reason to in this example, but they are useful when we use generic types and traits.

    let square = Rectangle::square(10);
    dbg!(&square);
    dbg!(square.width()); // note that when we use the parentheses, Rust knows that we're referencing the method on the struct, rather than the property "width" that the struct has.
    println!(square.width) // The opposite is true for reading the property - absence of parentheses means Rust knows to look for the property.

   

}

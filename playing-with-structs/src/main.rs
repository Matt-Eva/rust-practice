fn main() {
    // This program calculates the area of a rectangle:

    let rect1 = Rectangle {
        width: 30, 
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // use a reference so that main can maintain
        // ownership of rect1
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // prints in vertical format.
    // dbg!(rect1); // this would cause a move
    // dbg! can also be used to print out a value using debug format. However
    // it takes ownership of an expression, prints the file and line number of
    // where this macro occurs along with the resulting value of the referenced
    // expression, and returns ownership of the value.
    dbg!(&rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // move does not occur because scale is stack allocated. But, width gets a valid value because dbg! returns the result.
        height: 50,
    };
    dbg!(&rect2);
    dbg!(&scale);

    let name = String::from("john");
    let john = Person {
        name: dbg!(name), // this results in a move, because String types are 
                          // heap allocated.
    };
    // dbg!(&name); // get a move error, because name has been moved to scope
    // of dbg! from line 34.

}

#[derive(Debug)] // allows us to derive the Debug trait and print our Rectangle instance using debug formatting.
struct Rectangle {
    width: u32,
    height: u32,
}

struct Person {
    name: String,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

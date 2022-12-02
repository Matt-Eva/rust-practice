fn main() {
    let number = 7;

    if number < 5 {
        // condition must be bool - rust doesn't convert non-bool types to bool (good!).
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let x = 6;

    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if in let

    let condition = true;
    let y = if condition { 5 } else { 6 }; // similar to ternary

    println!("{y}");

    // we'll get an error if the types are mismatched
    // let con = true;
    // let z = if con {3} else {"3"}; // throws compiler error
    // // rust compiler must know at compile time the type of
    // // a variable.

    // println!("{z}")

    // LOOPS

    // Rust as three kinds of loops - loop, while, and for.

    // loop { // this loop will run continuously until manual stopped in terminal
    //     println!("again!")
    // }

    // we can use a break statement to exit a loop:

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // note that we place the values we want return immediately following the break statement.
                               // break // if we don't want a return value from our loop, we just include the break statement - the loop returns '()', which is the unit value.
        }
    };

    println!("the result is {result}");
    println!("the counter is {counter}");

    // LOOP LABELS

    // we can give labels to our loops to more easily keep track of
    // and interact with specific loops - e.g. when working with
    // nested loops:

    let mut new_count = 0;

    'counting_up: loop {
        // note that the label starts with a single '
        println!("new_count = {new_count}");
        let mut remaining = 10;
        loop {
            println!("remaining == {remaining}");
            if remaining == 9 {
                break; // specifies to break current loop.
            }
            if new_count == 2 {
                break 'counting_up; // specifies to break outer loop.
            }
            remaining -= 1;
        }
        new_count += 1;
    }
    println!("End new_count = {new_count}");

    // CONDITIONAL LOOPS - WHILE:

    // rust while loops work very similarly to while
    // loops in other languages, like javascript.

    let mut value = 3;

    while value != 0 {
        // checks condition before running block
        println!("{value}!");

        value -= 1; // modifies counter for condition.
    }
    // eliminates a lot of nesting!

    println!("LIFTOFF!!");

    // LOOPING THROUGH A COLLECTION

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1
    }
    // using a while loop to loop through our collection.
    // This is error prone - if we get our condition wrong
    // this code could throw a panic.
    // It also requires an extra step of a conditional check for
    // each iteration through the loop.

    // FOR LOOPS

    // Rust has a for loop very similar to JavaScript

    let b = [1, 2, 3, 4, 5];

    for element in b {
        println!("the value is: {element}");
    }
    // this is the prefered way to loop through a construct (array)

    // we can also use for loops with a range to loop a certain
    // number of times:

    for number in (1..4).rev() {
        // non inclusive range - .rev() reverses order. inclusive range uses (..=)
        println!("{number}!");
    }
    println!("LIFTOFF!!");

    // practice funcs:
    let temp = fahr_to_cel(50);
    println!("{temp}");

    for num in 1..=12 {
        gen_fib(num);
    }
}

fn fahr_to_cel(num: i32) -> i32 {
    (num - 32) * 5 / 9
}

fn gen_fib(num: i32) -> i32 {
    if num <= 1 {
        return 0;
    } else {
        let mut fib = 1;
        let mut prev_fib = 0;
        for _val in 2..num {
            let current_fib = fib;
            fib = current_fib + prev_fib;
            prev_fib = current_fib;
        }
        println!("{fib}");
        return fib;
    }
}

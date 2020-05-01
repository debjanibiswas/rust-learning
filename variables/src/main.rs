fn main() {
    let x = 5.0;  // with no specification, default type is f64
    let y: f32 = 3.0;  // specifying the type 
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // changing an immutable variable
    let x = x + 1.5;
    println!("The value of x+1.5 is: {}", x);

    function_parameter(40);

    // Expressions example 
    let a = {
        let b = 3;
        b + 1   // there is no semi-colon after an expression -- or it is a statement and will not return a value
    };  

    println!("The value of a is: {}", a);

    // Using a function returning a value
    let c = plus_one(5);
    println!("The value of plus_one is: {}", c);

    // Example with Control flow (if else statements)
    if c < 2 {
        println!("Number less than 50!");
    }
    else {
        println!("Number is not less than 50!");
    }

    // An example with a loop
    let mut counter = 0;
    let result = loop {
        counter = counter + 1;

        if counter == 10 {
            break counter * 2; // The loop will break when counter is 10, and this value will be in result
        }
    }; // semicolon indicates assignment of value to result

    println!("Result from loop is: {}", result);

    // An example with reverse and range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}


// Example of a function parameter
fn function_parameter(x: i32) { // function signatures require type annotations
    println!("The value of x is: {}", x);
}

// Example of a function with a return value
fn plus_one(x: i32) -> i32 {  
    x + 1 // again, no semi-colon after the expression that will return a value
}
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
}


// Example of a function parameter
fn function_parameter(x: i32) { // function signatures require type annotations
    println!("The value of x is: {}", x);
}

// Example of a function with a return value
fn plus_one(x: i32) -> i32 {  
    x + 1 // again, no semi-colon after the expression that will return a value
}
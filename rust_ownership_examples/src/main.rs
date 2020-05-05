fn main() {
    // immutable string
    let s = "hello";

    println!("The value of the string is: {}", s);
    
    // Mutable string (value stored in the heap)
    let s = String::from("helloo");
    let s2 = s.clone();  // need to use clone() to make deep copy
    println!("The value of the string is: {}", s);

    // Trying to change borrowed values
    let mut s3 = String::from("hello");
    println!("Changed string from borrowed values: {:?}", change(&mut s3));
    
    let s = String::from("helloo");
    let slice = &s[0..2];
    println!("{}",slice);
}

fn change(some_string: &mut String) {
    some_string.push_str(",world!");
}

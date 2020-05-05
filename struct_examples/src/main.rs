fn main() {
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_count: u64,
    }

    // The entire instance should be mutable
    let mut user1 = User {
        email: String::from("email@example.com"),
        username: String::from("username1"),
        active: true,
        sign_in_count: 1,
    };

    println!("The old username was: {} ", user1.username);
    user1.username = String::from ("another_username");
    println!("The new username was: {} ", user1.username);

    // Struct update syntax -- reusing values from an old instance
    let user2 = User {
        email: String::from("email2@example.com"),
        username: String::from("username2"),
        ..user1
    };
    println!("-------- Struct Update Example Output ------- ");
    println!("user2 email is: {}", user2.email);
    println!("user2 username is: {}", user2.username);
    println!("user2 sign_in_count: {}", user2.sign_in_count);
    println!("user2 active_status: {}", user2.active);

    // Tuple Structs with no named fields
    struct Color(i32, i32, i32);
    let white = Color(255, 253, 255);
    println!("");
    println!("--------------  Tuple Struct Example Output --------");
    println!("Color white R value is: {}", white.1);
}

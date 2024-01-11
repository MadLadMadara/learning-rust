struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // tuple struct
struct Point(i32, i32, i32); // tuple struct

struct AlwaysEqual;



fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("new email");

    let user2 = User {
        username: String::from("someusername1234"),
        ..user1 // struct update syntax
    };

    println!("user1.email: {}", user2.email);

    // tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like struct
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,
        sign_in_count: 1,
    }
}


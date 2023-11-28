struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32); // tuple struct
struct Point(i32, i32, i32); // tuple struct

fn main() {
    let user1 = build_user(
        String::from("y.hotta.engineer@gmail.com"),
        String::from("y-hotta"),
    );
    println!("user: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("user: {}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}

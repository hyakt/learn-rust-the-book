struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct
struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        username: String::from("anothername576"),
        ..user1
    };

    Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

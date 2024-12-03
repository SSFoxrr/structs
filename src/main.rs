struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("diffemail@hi.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("anothaone.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("example.com"),
        ..user1
    };

    //tuple structs
    struct color(i32, i32, i32);
    struct point(i32, i32, i32);

    fn tester() {
        let black = color(0, 0, 0);
        let origin = point(0, 0, 0);
    }

    //unit-like structs without fields
    struct AlwaysEqual;

    fn swat() {
        let subject = AlwaysEqual;
    }

}

//error[E0106]: missing lifetime specifier
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: String,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
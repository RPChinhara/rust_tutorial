struct User {
    _active: bool,
    _username: String,
    _email:String,
    _sign_in_count: u64
}

fn main() {
    let user1 = build_user(String::from("someone123@mail.com"), String::from("someone123"));

    let _user2 = User {
        _email: String::from("anotherexample@mail.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User { 
        _active: true, 
        _username: username, 
        _email: email, 
        _sign_in_count: 1 
    }
}
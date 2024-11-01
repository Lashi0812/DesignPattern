mod password;
mod user_repository;
mod authenticator;

use password::PasswordHasher;
use user_repository::UserRepository;
use authenticator::Authenticator;

fn main() {
    let password_hasher = PasswordHasher;
    let user_repo = UserRepository;
    let auth = Authenticator::new(&password_hasher, &user_repo);

    let username = "user1";
    let password = "password123";

    match auth.authenticate(username, password) {
        Ok(true) => println!("Authentication successful!"),
        Ok(false) => println!("Invalid credentials."),
        Err(err) => println!("Error: {}", err),
    }
}

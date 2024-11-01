use crate::password::PasswordHasher;
use crate::user_repository::UserRepository;

/// Authenticates users using `PasswordHasher` and `UserRepository`.
pub struct Authenticator<'a> {
    password_hasher: &'a PasswordHasher,
    user_repo: &'a UserRepository,
}

impl<'a> Authenticator<'a> {
    /// Creates a new `Authenticator` with references to `PasswordHasher` and `UserRepository`.
    pub fn new(password_hasher: &'a PasswordHasher, user_repo: &'a UserRepository) -> Self {
        Authenticator {
            password_hasher,
            user_repo,
        }
    }

    /// Authenticates a user by verifying the password against stored data.
    pub fn authenticate(&self, username: &str, password: &str) -> Result<bool, &'static str> {
        if let Some((_, hashed_password)) = self.user_repo.get_user(username) {
            return self
                .password_hasher
                .verify_password(password, &hashed_password)
                .map_err(|_| "Failed to verify password");
        }
        Err("User not found")
    }
}

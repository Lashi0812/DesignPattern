use bcrypt::{hash, verify};

pub struct PasswordHasher;

impl PasswordHasher {
    /// Hashes a password using bcrypt.
    pub fn hash_password(&self, password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, 4)
    }

    /// Verifies a password against a hashed password.
    pub fn verify_password(&self, password: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hashed)
    }
}

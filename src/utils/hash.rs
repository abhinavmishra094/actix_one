use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;

pub fn hash_password(password: &str) -> String {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    Pbkdf2
        .hash_password(password, salt.as_ref())
        .unwrap()
        .to_string()
}

pub fn verify_password(password_hash: &str, password: &str) -> bool {
    let password_hash = PasswordHash::new(password_hash).unwrap();

    Pbkdf2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}

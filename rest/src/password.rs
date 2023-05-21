use argon2::{
    password_hash::{rand_core::{OsRng, RngCore}, SaltString},
    PasswordHash, PasswordHasher, PasswordVerifier, Argon2,
    Error
};

pub fn hash(password: &[u8]) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default().hash_password(password, &salt) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(e.to_string())
    }
}

pub fn verify(password: &[u8], hash: &str) -> bool {
    if let Ok(hash) = PasswordHash::new(hash) {
        Argon2::default().verify_password(password, &hash).is_ok()
    } else { false }
}

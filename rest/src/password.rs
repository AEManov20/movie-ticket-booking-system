use argon2::{
    password_hash::{rand_core::OsRng, SaltString, Error},
    PasswordHash, PasswordHasher, PasswordVerifier, Argon2,
};

pub fn hash(password: &[u8]) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default().hash_password(password, &salt)?.to_string())
}

pub fn verify(password: &[u8], hash: &str) -> bool {
    if let Ok(hash) = PasswordHash::new(hash) {
        Argon2::default().verify_password(password, &hash).is_ok()
    } else { false }
}

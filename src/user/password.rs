use argon2::{
    Argon2,
    password_hash::{
        PasswordHasher,
        PasswordHash,
        PasswordVerifier,
        SaltString,
    },
};

use rand_core::OsRng;
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hash)
}
pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;

    Ok(
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    )
}
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Hash the provided password using Argon2.
pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let hash_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Error hashing password: {}", e);
            argon2::Error::AlgorithmInvalid
        })?;

    Ok(hash_password.to_string())
}

/// Verify that a password matches the provided çå.
pub fn verify_password(password_hash: &str, password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Error hashing password: {}", e);
            return false;
        }
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "super_secret_password";
        let hash = hash_password(password).expect("Failed to hash password");

        // Verifying that the hashed password matches the original one
        assert!(verify_password(&hash, password));
        // A wrong password should return false.
        assert!(!verify_password(&hash, "wrong_password"));
    }

    #[test]
    fn test_argon2_jvm_verify() {
        let password = "mySecretPassword";
        let hash = "$argon2i$v=19$m=65536,t=2,p=1$vNVL5PZ1hRwgLUlGmCQVTA$fg1d0/f8pdtMnzQTeh2YE6R0E8vfqMOQOs5k6Y22Qi0";
        assert!(verify_password(hash, password));
    }
}

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Email ou ID de l'utilisateur
    pub id: i32,
    pub role: String, // ex: "admin" ou "viewer"
    pub exp: usize,   // Date d'expiration
}

// -------------------------------------------------------------------------
// 1. GESTION DES TOKENS JWT
// -------------------------------------------------------------------------

pub fn create_jwt(user_id: i32, email: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Date valide")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        id: user_id.to_owned(),
        role: role.to_owned(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_defaut".into());
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

// -------------------------------------------------------------------------
// 2. GESTION DU HACHAGE DES MOTS DE PASSE (Argon2)
// -------------------------------------------------------------------------

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, parsed_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(parsed_hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
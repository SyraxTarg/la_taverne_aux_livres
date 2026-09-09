use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

fn valider_extension_email(email: &str) -> Result<(), validator::ValidationError> {
    if let Some(domain) = email.split('@').nth(1) {
        if domain.contains('.') && !domain.ends_with('.') {
            return Ok(());
        }
    }
    let mut err = validator::ValidationError::new("extension_invalide");
    err.message = Some("L'email doit contenir un domaine valide (ex: .com, .fr)".into());
    Err(err)
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct RegisterDto {
    #[validate(
        email(message = "Format d'email invalide"),
        custom(function = "valider_extension_email")
    )]
    pub email: String,
    pub password: String,
    pub role_id: i32,
}


#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginDto {
    #[validate(
        email(message = "Format d'email invalide"),
        custom(function = "valider_extension_email")
    )]
    pub email: String,
    pub password: String,
}

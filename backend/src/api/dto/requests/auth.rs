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
    pub username: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_register_dto() {
        let dto = RegisterDto {
            email: "test.lecteur@taverne.fr".to_string(),
            password: "super_secret_password".to_string(),
            username: "gandalf".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_register_dto_invalid_email_format() {
        let dto = RegisterDto {
            email: "pas_un_email".to_string(),
            password: "password".to_string(),
            username: "frodo".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_register_dto_email_without_extension() {
        let dto = RegisterDto {
            email: "user@localhost".to_string(),
            password: "password".to_string(),
            username: "aragorn".to_string(),
        };
        let res = dto.validate();
        assert!(res.is_err());
        let err_str = res.unwrap_err().to_string();
        assert!(err_str.contains("extension_invalide") || err_str.contains("email"));
    }

    #[test]
    fn test_register_dto_email_ending_with_dot() {
        let dto = RegisterDto {
            email: "user@domain.".to_string(),
            password: "password".to_string(),
            username: "legolas".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_valid_login_dto() {
        let dto = LoginDto {
            email: "admin@taverne.com".to_string(),
            password: "secret_password".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_invalid_login_dto_email() {
        let dto = LoginDto {
            email: "@domaine_incomplet.com".to_string(),
            password: "password".to_string(),
        };
        assert!(dto.validate().is_err());
    }
}

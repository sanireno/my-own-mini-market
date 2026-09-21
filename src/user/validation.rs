use crate::app_error::AppError;
use crate::user_models::RegisterUser;

pub fn normalize_email(email: &str) -> String {
    // Preserve the existing case-sensitive account identity until a dedicated migration.
    email.trim().to_string()
}

pub fn validate_registration(mut user: RegisterUser) -> Result<RegisterUser, AppError> {
    user.username = user.username.trim().to_string();
    user.email = normalize_email(&user.email);
    if !(1..=100).contains(&user.username.chars().count())
        || user.username.chars().any(char::is_control)
    {
        return Err(AppError::Validation(
            "username must contain 1 to 100 characters without control characters".to_string(),
        ));
    }
    if !valid_email(&user.email) {
        return Err(AppError::Validation(
            "email must be a valid ASCII address of at most 254 bytes".to_string(),
        ));
    }
    if !(8..=128).contains(&user.password.chars().count())
        || user.password.chars().all(char::is_whitespace)
    {
        return Err(AppError::Validation(
            "password must contain 8 to 128 characters and must not be only whitespace".to_string(),
        ));
    }
    Ok(user)
}

fn valid_email(email: &str) -> bool {
    if !email.is_ascii() || email.len() > 254 {
        return false;
    }
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    // Support ordinary dot-atom addresses; quoted local parts and IP literals are excluded.
    !local.is_empty()
        && local.len() <= 64
        && !local.starts_with('.')
        && !local.ends_with('.')
        && !local.contains("..")
        && local
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b".!#$%&'*+-/=?^_`{|}~".contains(&c))
        && domain.contains('.')
        && domain.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'-')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user() -> RegisterUser {
        RegisterUser {
            username: "  Рене  ".to_string(),
            email: "  Rene+shop@example.com  ".to_string(),
            password: " password ".to_string(),
        }
    }

    #[test]
    fn normalizes_identity_without_changing_password_or_email_case() {
        let validated = validate_registration(user()).unwrap();
        assert_eq!(validated.username, "Рене");
        assert_eq!(validated.email, "Rene+shop@example.com");
        assert_eq!(validated.password, " password ");
    }

    #[test]
    fn rejects_empty_long_and_control_character_names() {
        for name in ["   ".to_string(), "a".repeat(101), "a\nb".to_string()] {
            let mut input = user();
            input.username = name;
            assert!(validate_registration(input).is_err());
        }
    }

    #[test]
    fn rejects_invalid_email_addresses() {
        for email in [
            "",
            "abc",
            "@example.com",
            "a@@example.com",
            "a b@example.com",
            "a..b@example.com",
            "a@-example.com",
            "a@example..com",
            "a@localhost",
            "я@example.com",
        ] {
            let mut input = user();
            input.email = email.to_string();
            assert!(validate_registration(input).is_err(), "{email}");
        }
        assert!(!valid_email(&format!("{}@example.com", "a".repeat(65))));
        assert!(!valid_email(&format!("a@{}.com", "a".repeat(64))));
        assert!(!valid_email(&format!(
            "{}@{}.{}.{}.com",
            "a".repeat(64),
            "b".repeat(63),
            "c".repeat(63),
            "d".repeat(63)
        )));
    }

    #[test]
    fn enforces_password_bounds_in_characters() {
        for password in [
            "".to_string(),
            "1234567".to_string(),
            " ".repeat(8),
            "a".repeat(129),
        ] {
            let mut input = user();
            input.password = password;
            assert!(validate_registration(input).is_err());
        }
        for password in ["пароль12".to_string(), "a".repeat(128)] {
            let mut input = user();
            input.password = password;
            assert!(validate_registration(input).is_ok());
        }
    }
}

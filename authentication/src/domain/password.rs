use std::fmt;

pub const MIN_PASSWORD_LENGTH: usize = 16;
pub const MAX_PASSWORD_LENGTH: usize = 64;

struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self, PasswordInvalidError> {
        return Self::validate_requirements(password);
    }

    fn validate_requirements(password: String) -> Result<Self, PasswordInvalidError> {
        let password_len = password.chars().count();
        if password_len < MIN_PASSWORD_LENGTH || password_len > MAX_PASSWORD_LENGTH {
            PasswordInvalidError::PasswordInvalidLengthError {};
        }
        Ok(Password(password))
    }
}

#[derive(Debug)]
pub enum PasswordInvalidError {
    PasswordInvalidLengthError {},
    PasswordInvalidError { message: String },
}

impl fmt::Display for PasswordInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *&self {
            PasswordInvalidError::PasswordInvalidLengthError {} => f.write_str(&format!(
                "Password must be between {} and {} characters",
                MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH
            )),
            PasswordInvalidError::PasswordInvalidError { message } => f.write_str(&message),
        }
    }
}

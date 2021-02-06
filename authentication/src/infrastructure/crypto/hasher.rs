use crate::domain::hasher::{Hasher, HasherError};
use sodiumoxide::crypto::pwhash::argon2id13;

pub struct ArgonHasher {}

impl Hasher for ArgonHasher {
    fn hash(&self, password: String) -> Result<String, HasherError> {
        let result = argon2id13::pwhash(
            &password.as_bytes(),
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        );
        match result {
            Ok(hashed_password_result) => match std::str::from_utf8(&hashed_password_result.0) {
                Ok(hashed_password_utf8) => {
                    Ok(String::from(hashed_password_utf8.trim_end_matches('\u{0}')))
                }
                Err(e) => Err(HasherError::from(e)),
            },
            Err(e) => Err(HasherError::from(e)),
        }
    }
}

impl From<()> for HasherError {
    fn from(_: ()) -> Self {
        HasherError::HasherError {
            message: String::from("Error hashing credentials"),
        }
    }
}

impl From<std::str::Utf8Error> for HasherError {
    fn from(cause: std::str::Utf8Error) -> Self {
        HasherError::HasherError {
            message: format!("{}", cause),
        }
    }
}

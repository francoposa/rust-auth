use crate::domain::hasher::{Hasher, HasherError};
use sodiumoxide::crypto::pwhash::argon2id13;

pub struct ArgonHasher {}

impl ArgonHasher {
    fn hash(password: String) -> Result<String, HasherError> {
        let result = argon2id13::pwhash(
            &password.as_bytes(),
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        );
        match result {
            Ok(hashed_password) => Ok(String::from(String::from_utf8_lossy(&hashed_password.0))),
            Err(()) => Err(HasherError::from(())),
        }
    }
}

impl From<()> for HasherError {
    fn from(_: ()) -> Self {
        HasherError::HasherError {
            message: String::from(""),
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

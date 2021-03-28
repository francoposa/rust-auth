use chrono::{DateTime, Utc};
use std::fmt;
use uuid::Uuid;

pub const MIN_USERNAME_LENGTH: usize = 8;
pub const MAX_USERNAME_LENGTH: usize = 64;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        let now = Utc::now();
        User {
            id: Uuid::new_v4(),
            username,
            email,
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug)]
pub enum UsernameInvalidError {
    UsernameInvalidLengthError {},
    UsernameInvalidError { message: String },
}

impl fmt::Display for UsernameInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *&self {
            UsernameInvalidError::UsernameInvalidLengthError {} => f.write_str(&format!(
                "Username must be between {} and {} characters",
                MIN_USERNAME_LENGTH, MAX_USERNAME_LENGTH
            )),
            UsernameInvalidError::UsernameInvalidError { message } => f.write_str(&message),
        }
    }
}

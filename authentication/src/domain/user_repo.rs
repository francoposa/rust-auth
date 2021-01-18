use crate::domain::user::User;
use std::fmt;

pub trait UserRepo {
    fn create(&self, user: User) -> Result<User, UserRepoError>;
}

#[derive(Debug)]
pub enum UserRepoError {
    UserAlreadyExists { field: String, value: String },
}

impl fmt::Display for UserRepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *&self {
            UserRepoError::UserAlreadyExists { field, value } => {
                f.write_str(&format!("User with {}={} already exists", field, value))
            }
        }
    }
}

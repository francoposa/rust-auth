use crate::domain::user::User;
use async_trait::async_trait;
use std::fmt;

#[async_trait]
pub trait UserRepo {
    async fn create(&self, user: User) -> Result<User, UserRepoError>;
}

#[derive(Debug)]
pub enum UserRepoError {
    UserAlreadyExistsError { field: String, value: String },
    UserRepoError { message: String },
}

impl fmt::Display for UserRepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *&self {
            UserRepoError::UserAlreadyExistsError { field, value } => {
                f.write_str(&format!("User with {}={} already exists", field, value))
            }
            UserRepoError::UserRepoError { message } => f.write_str(&message),
        }
    }
}

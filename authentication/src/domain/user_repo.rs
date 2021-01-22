use crate::domain::user::User;
use async_trait::async_trait;
use std::fmt;

#[async_trait]
pub trait UserRepo {
    async fn create(&self, user: User) -> sqlx::Result<()>;
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

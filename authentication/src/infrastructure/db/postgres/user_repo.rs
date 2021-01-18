use crate::domain::{
    user::User,
    user_repo::{UserRepo, UserRepoError},
};

pub struct PGUserRepo {}

impl UserRepo for PGUserRepo {
    fn create(&self, user: User) -> Result<User, UserRepoError> {
        Ok(user)
    }
}

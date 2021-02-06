use crate::domain::{
    hasher::{Hasher, HasherError},
    user::User,
    user_repo::{UserRepo, UserRepoError},
};
use async_trait::async_trait;
use sqlx;
use sqlx::postgres::PgPool;

pub struct PGUserRepo {
    conn_pool: PgPool,
    hasher: Box<dyn Hasher + Sync + Send>,
}

impl PGUserRepo {
    pub fn new(conn_pool: PgPool, hasher: Box<dyn Hasher + Sync + Send>) -> Self {
        PGUserRepo { conn_pool, hasher }
    }
}

#[async_trait]
impl UserRepo for PGUserRepo {
    async fn create(&self, user: User, password: String) -> Result<User, UserRepoError> {
        let hashed_password: String = self.hasher.hash(password)?;
        let result = sqlx::query_as!(
            User,
            "INSERT INTO authn_user (
	        id, username, email, password, enabled, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, username, email, enabled, created_at, updated_at",
            user.id,
            user.username,
            user.email,
            hashed_password,
            user.enabled,
            user.created_at,
            user.updated_at,
        )
        .fetch_one(&self.conn_pool)
        .await;

        match result {
            Ok(created_user) => Ok(created_user),
            Err(e) => Err(UserRepoError::from(e)),
        }
    }
}

impl From<sqlx::Error> for UserRepoError {
    fn from(cause: sqlx::Error) -> Self {
        UserRepoError::UserRepoError {
            message: format!("{}", cause),
        }
    }
}

impl From<HasherError> for UserRepoError {
    fn from(cause: HasherError) -> Self {
        UserRepoError::UserRepoError {
            message: format!("{}", cause),
        }
    }
}

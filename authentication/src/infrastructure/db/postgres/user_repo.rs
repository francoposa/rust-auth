use crate::domain::{
    user::User,
    user_repo::{UserRepo, UserRepoError},
};
use async_trait::async_trait;
use sqlx;
use sqlx::postgres::PgPool;

pub struct PGUserRepo {
    conn_pool: PgPool,
}

impl PGUserRepo {
    fn new(conn_pool: PgPool) -> PGUserRepo {
        PGUserRepo { conn_pool }
    }
}

#[async_trait]
impl UserRepo for PGUserRepo {
    async fn create(&self, user: User) -> sqlx::Result<()> {
        let result = sqlx::query!(
            "INSERT INTO authn_user (
	        id, username, email, password, enabled, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, username, email, enabled, created_at, updated_at",
            user.id,
            user.username,
            user.email,
            "",
            user.enabled,
            user.created_at,
            user.updated_at,
        )
        .fetch_one(&self.conn_pool)
        .await;

        result
    }
}

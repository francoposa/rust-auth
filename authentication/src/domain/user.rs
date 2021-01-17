use chrono::{DateTime, Utc};
use uuid::Uuid;

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
    pub fn new(username: String, email: String) -> User {
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

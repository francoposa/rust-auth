use sqlx::postgres::PgPoolOptions;
use std::error::Error;

mod domain;
mod infrastructure;

use crate::infrastructure::db;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let config = db::postgres::config::Config {
        host: String::from("localhost"),
        port: 5432,
        username: String::from("postgres"),
        password: String::from(""),
        database: String::from("authentication"),
        application_name: String::from("authentication"),
        connect_timeout_seconds: 5,
        ssl_mode: String::from("disable"),
    };

    let db_url = &config.url();
    let pg_pool = PgPoolOptions::new().connect(db_url).await?;

    let result = sqlx::query_as!(SelectIntResult, "SELECT 1 AS result")
        .fetch_one(&pg_pool)
        .await?;

    println!("{:?}", result.result);

    let user = domain::user::User::new(String::from("test"), String::from("test@test.com"));

    let user_repo: Box<dyn domain::user_repo::UserRepo> =
        Box::new(db::postgres::user_repo::PGUserRepo {});

    user_repo.create(user);

    Ok(())
}

#[derive(Debug)]
struct SelectIntResult {
    result: Option<i32>,
}

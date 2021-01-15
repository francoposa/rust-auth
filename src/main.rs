use sqlx::postgres::PgPoolOptions;

mod db;

use crate::db::pg;

#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");

    let db_url = &pg::db_url();

    let pg_pool = PgPoolOptions::new().connect(db_url).await?;

    let result = sqlx::query_as!(SelectIntResult, "SELECT 1 AS result")
        .fetch_one(&pg_pool)
        .await?;

    println!("{:?}", result.result);

    Ok(())
}

#[derive(Debug)]
struct SelectIntResult {
    result: Option<i32>,
}

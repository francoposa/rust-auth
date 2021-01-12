use sqlx::postgres::PgPoolOptions;

#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");

    let pool = PgPoolOptions::new()
        .connect("postgres://postgres@localhost/authentication_identity_user_mgmt")
        .await?;

    #[derive(Debug)]
    struct Select1Result {
        result: Option<i32>,
    };

    let result = sqlx::query_as!(Select1Result, "SELECT 1 AS result")
        .fetch_one(&pool)
        .await?;

    println!("{:?}", result.result);

    Ok(())
}

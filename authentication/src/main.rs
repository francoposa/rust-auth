use actix_web::{rt::System, web, App, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

mod application;
mod domain;
mod infrastructure;
use crate::infrastructure::crypto;
use crate::infrastructure::db;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    sodiumoxide::init().unwrap();
    let hasher = Box::new(crypto::hasher::ArgonHasher {});

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
    let conn_pool = PgPoolOptions::new().connect(db_url).await?;

    let user_repo: Box<dyn domain::user_repo::UserRepo> =
        Box::new(db::postgres::user_repo::PGUserRepo::new(conn_pool, hasher));

    let user = domain::user::User::new(String::from("test"), String::from("test@test.com"));
    //let created_user = user_repo.create(user, String::from("test_pass")).await;
    //println!("{:?}", created_user.unwrap());

    let sys = System::new("http-server");

    HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    sys.run()?;

    Ok(())
}

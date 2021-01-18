#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub database: String,
    pub application_name: String,
    pub connect_timeout_seconds: i32,
    pub ssl_mode: String,
}

impl Config {
    pub fn url(&self) -> String {
        let auth = if self.username.len() > 0 || self.password.len() > 0 {
            format!("{}:{}@", self.username, self.password)
        } else {
            String::from("")
        };

        format!(
            "postgres://{}{}:{}/{}?application_name={}&connect_timeout={}&sslmode={}",
            auth,
            self.host,
            self.port,
            self.database,
            self.application_name,
            self.connect_timeout_seconds,
            self.ssl_mode
        )
    }
}

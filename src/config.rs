use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16
}

impl Config {
    pub fn init() -> Config {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_maxage = env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        Config {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>().expect("JWT_MAXAGE must be i64"),
            port: 8081
        }
    }
}
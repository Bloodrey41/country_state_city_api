use mysql::*;
use std::env;

pub struct DBConnection;

impl DBConnection {
    pub fn init() -> Result<PooledConn> {
        let host = env::var("DB_HOST").unwrap_or(String::from("localhost"));
        let user = env::var("DB_USER").unwrap_or(String::from("root"));
        let password = env::var("DB_PASSWORD").unwrap_or(String::from(""));
        let database = env::var("DB_DATABASE").unwrap_or(String::from("world"));

        let pool = Pool::new(OptsBuilder::new()
            .ip_or_hostname(Some(host))
            .user(Some(user))
            .pass(Some(password))
            .db_name(Some(database))
            .ssl_opts(SslOpts::default())
        )?;

        let conn = pool.get_conn()?;

        Ok(conn)
    }
}

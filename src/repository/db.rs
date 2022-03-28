use mysql::*;

pub struct DBRepository;

impl DBRepository {
    pub fn init() -> Result<PooledConn> {
        let pool = Pool::new(OptsBuilder::new()
            .user(Some("root"))
            .db_name(Some("world"))
        )?;

        let conn = pool.get_conn()?;

        Ok(conn)
    }
}

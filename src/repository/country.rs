use mysql::*;
use mysql::prelude::*;

use crate::db::DBConnection;
use crate::model::country::Country;

pub struct CountryRepository {
    conn: PooledConn,
}

impl CountryRepository {
    pub fn new() -> Self {
        Self { conn: DBConnection::init().unwrap() }
    }

    pub fn find(mut self) -> Result<Vec<Country>> {
        let sql = "SELECT * FROM countries";

        let countries = self.conn.query_map(
            sql,
            |(id, name)| {
                Country::new(id, name)
            },
        )?;

        Ok(countries)
    }

    pub fn find_by_id(mut self, id: u32) -> Result<Option<Country>> {
        let sql = "SELECT * FROM countries WHERE id = ?";

        let countries = self.conn.exec_map(
            sql,
            vec![id],
            |(id, name)| {
                Country::new(id, name)
            },
        )?;

        match countries.first() {
            Some(country) => Ok(Some(country.to_owned())),
            None => Ok(None)
        }
    }
}

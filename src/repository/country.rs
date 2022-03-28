use mysql::*;
use mysql::prelude::*;

use crate::repository::db::DBRepository;
use crate::model::country::Country;

pub struct CountryRepository {
    conn: PooledConn,
}

impl CountryRepository {
    pub fn new() -> Self {
        let country_repository = Self { conn: DBRepository::init().unwrap() };
        country_repository
    }

    pub fn find(mut self) -> Result<Vec<Country>> {
        let sql = "SELECT * FROM countries";

        let result = self.conn.query_map(
            sql,
            |(id, name)| {
                Country::new(id, name)
            },
        );

        match result {
            Ok(countries) => Ok(countries),
            Err(err) => Err(err),
        }
    }

    pub fn find_by_id(mut self, id: u32) -> Result<Option<Country>> {
        let sql = "SELECT * FROM countries WHERE id = ?";

        let result = self.conn.exec_map(
            sql,
            vec![id],
            |(id, name)| {
                Country::new(id, name)
            },
        );

        match result {
            Ok(countries) => {
                match countries.first() {
                    Some(country) => Ok(Some(country.to_owned())),
                    None => Ok(None)
                }
            },
            Err(err) => Err(err),
        }
    }
}

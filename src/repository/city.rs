use mysql::*;
use mysql::prelude::*;

use crate::db::DBConnection;
use crate::model::city::City;

pub struct CityRepository {
    conn: PooledConn,
}

impl CityRepository {
    pub fn new() -> Self {
        Self { conn: DBConnection::init().unwrap() }
    }

    pub fn find(mut self) -> Result<Vec<City>> {
        let sql = "SELECT * FROM cities";

        let cities = self.conn.query_map(
            sql,
            |(id, name, state_id, country_id)| {
                City::new(id, name, state_id, country_id)
            },
        )?;

        Ok(cities)
    }

    pub fn find_by_id(mut self, id: u32) -> Result<Option<City>> {
        let sql = "SELECT * FROM cities WHERE id = ?";

        let cities = self.conn.exec_map(
            sql,
            vec![id],
            |(id, name, state_id, country_id)| {
                City::new(id, name, state_id, country_id)
            },
        )?;

        match cities.first() {
            Some(cities) => Ok(Some(cities.to_owned())),
            None => Ok(None)
        }
    }


    pub fn find_by_state(mut self, state_id: u32) -> Result<Option<Vec<City>>> {
        let sql = "SELECT * FROM cities WHERE state_id = ?";

        let cities = self.conn.exec_map(
            sql,
            vec![state_id],
            |(id, name, state_id, country_id)| {
                City::new(id, name, state_id, country_id)
            },
        )?;

        match cities.first() {
            Some(_) => Ok(Some(cities)),
            None => Ok(None)
        }
    }

    pub fn find_by_country(mut self, country_id: u32) -> Result<Option<Vec<City>>> {
        let sql = "SELECT * FROM cities WHERE country_id = ?";

        let cities = self.conn.exec_map(
            sql,
            vec![country_id],
            |(id, name, state_id, country_id)| {
                City::new(id, name, state_id, country_id)
            },
        )?;

        match cities.first() {
            Some(_) => Ok(Some(cities)),
            None => Ok(None)
        }
    }
}

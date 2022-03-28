use mysql::*;
use mysql::prelude::*;

use crate::repository::db::DBRepository;
use crate::model::city::City;

pub struct CityRepository {
    conn: PooledConn,
}

impl CityRepository {
    pub fn new() -> Self {
        let city_repository = Self { conn: DBRepository::init().unwrap() };
        city_repository
    }

    pub fn find(mut self) -> Result<Vec<City>> {
        let sql = "SELECT * FROM cities";

        let result = self.conn.query_map(
            sql,
            |(id, name, state_id, country_id)| {
                City::new(id, name, state_id, country_id)
            },
        );

        match result {
            Ok(cities) => Ok(cities),
            Err(err) => Err(err),
        }
    }

    pub fn find_by_id(mut self, id: u32) -> Result<Option<City>> {
        let sql = "SELECT * FROM cities WHERE id = ?";

        let result = self.conn.exec_map(
            sql,
            vec![id],
            |(id, name, state_id, country_id)| {
                City::new(id, name, state_id, country_id)
            },
        );

        match result {
            Ok(cities) => {
                match cities.first() {
                    Some(cities) => Ok(Some(cities.to_owned())),
                    None => Ok(None)
                }
            },
            Err(err) => Err(err),
        }
    }

    pub fn find_by_state(mut self, state_id: u32) -> Result<Option<Vec<City>>> {
        let sql = "SELECT * FROM cities WHERE state_id = ?";

        let result = self.conn.exec_map(
            sql,
            vec![state_id],
            |(id, name, state_id, country_id)| {
                City::new(id, name, state_id, country_id)
            },
        );

        match result {
            Ok(cities) => {
                match cities.first() {
                    Some(_) => Ok(Some(cities)),
                    None => Ok(None)
                }
            },
            Err(err) => Err(err),
        }
    }

    pub fn find_by_country(mut self, country_id: u32) -> Result<Option<Vec<City>>> {
        let sql = "SELECT * FROM cities WHERE country_id = ?";

        let result = self.conn.exec_map(
            sql,
            vec![country_id],
            |(id, name, state_id, country_id)| {
                City::new(id, name, state_id, country_id)
            },
        );

        match result {
            Ok(cities) => {
                match cities.first() {
                    Some(_) => Ok(Some(cities)),
                    None => Ok(None)
                }
            },
            Err(err) => Err(err),
        }
    }
}

use mysql::*;
use mysql::prelude::*;

use crate::db::DBConnection;
use crate::model::state::State;

pub struct StateRepository {
    conn: PooledConn,
}

impl StateRepository {
    pub fn new() -> Self {
        Self { conn: DBConnection::init().unwrap() }
    }

    pub fn find(mut self) -> Result<Vec<State>> {
        let sql = "SELECT * FROM states";

        let states = self.conn.query_map(
            sql,
            |(id, name, country_id)| {
                State::new(id, name, country_id)
            },
        )?;

        Ok(states)
    }

    pub fn find_by_id(mut self, id: u32) -> Result<Option<State>> {
        let sql = "SELECT * FROM states WHERE id = ?";

        let states = self.conn.exec_map(
            sql,
            vec![id],
            |(id, name, country_id)| {
                State::new(id, name, country_id)
            },
        )?;

        match states.first() {
            Some(state) => Ok(Some(state.to_owned())),
            None => Ok(None)
        }
    }

    pub fn find_by_country(mut self, country_id: u32) -> Result<Option<Vec<State>>> {
        let sql = "SELECT * FROM states WHERE country_id = ?";

        let states = self.conn.exec_map(
            sql,
            vec![country_id],
            |(id, name, country_id)| {
                State::new(id, name, country_id)
            },
        )?;

        match states.first() {
            Some(_) => Ok(Some(states)),
            None => Ok(None)
        }
    }
}

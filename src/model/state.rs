use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct State {
    id: u32,
    name: String,
    country_id: u32
}

impl State {
    pub fn new(id: u32, name: String, country_id: u32) -> Self {
        let state = Self { id, name, country_id };
        state
    }
}

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct City {
    id: u32,
    name: String,
    state_id: u32,
    country_id: u32
}

impl City {
    pub fn new(id: u32, name: String, state_id: u32, country_id: u32) -> Self {
        let city = Self { id, name, state_id, country_id };
        city
    }
}

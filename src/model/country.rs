use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Country {
    id: u32,
    name: String,
}

impl Country {
    pub fn new(id: u32, name: String) -> Self {
        let country = Self { id, name };
        country
    }
}

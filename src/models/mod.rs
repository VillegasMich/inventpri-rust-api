use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub location: String,
}

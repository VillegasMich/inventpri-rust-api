use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub location: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatedItem {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub location: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovedItem {
    pub id: u32,
    pub location: String,
}

use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub name: String,
    pub price: u32,
    pub location: String
}
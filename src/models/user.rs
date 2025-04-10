use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) email: String,
}
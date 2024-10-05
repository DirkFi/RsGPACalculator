use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Course {
    pub id: usize,
    pub name: String,
    pub teacher: String,
    pub description: String,
    pub image: String,
    pub unit: i32,
}

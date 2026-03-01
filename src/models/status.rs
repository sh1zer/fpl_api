use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventStatus {
    pub status: Vec<StatusEntry>,
    pub leagues: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusEntry {
    pub bonus_added: bool,
    pub date: String,
    pub event: i32,
    pub points: String,
}

use serde::{Deserialize, Serialize};

/// Bonus point and scoring status across active gameweeks (events)
#[derive(Debug, Serialize, Deserialize)]
pub struct EventStatus {
    pub status: Vec<StatusEntry>,
    pub leagues: String,
}

/// Scoring and bonus status for a single date within a gameweek (event)
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusEntry {
    pub bonus_added: bool,
    pub date: String,
    pub event: i32,
    pub points: String,
}

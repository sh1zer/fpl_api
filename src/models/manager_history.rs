use serde::{Deserialize, Serialize};
use crate::models::picks::EntryHistory;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerHistory {
    pub current: Vec<EntryHistory>,
    pub past: Vec<PastHistory>,
    pub chips: Vec<ChipUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PastHistory {
    pub season_name: String,
    pub total_points: i32,
    pub rank: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChipUsage {
    pub name: String,
    pub time: String,
    pub event: i32,
}

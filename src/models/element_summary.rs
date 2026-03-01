use serde::{Deserialize, Serialize};
use crate::models::fixture::Fixture;
use crate::models::picks::EntryHistory;

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementSummary {
    pub fixtures: Vec<Fixture>,
    pub history: Vec<EntryHistory>,
    pub history_past: Vec<SeasonHistory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonHistory {
    pub season_name: String,
    pub total_points: i32,
    pub rank: u32,
}

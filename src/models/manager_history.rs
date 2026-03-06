use crate::models::picks::EntryHistory;
use serde::{Deserialize, Serialize};

/// A manager's full history: current season gameweek-by-gameweek, past seasons, and chip usage
#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerHistory {
    /// Gameweek-by-gameweek (event-by-event) history for the current season
    pub current: Vec<EntryHistory>,
    /// Overall stats for past seasons
    pub past: Vec<PastHistory>,
    pub chips: Vec<ChipUsage>,
}

/// A manager's summary stats for a previous season
#[derive(Debug, Serialize, Deserialize)]
pub struct PastHistory {
    pub season_name: String,
    pub total_points: i32,
    pub rank: i32,
}

/// A record of a chip played by a manager, including which gameweek (event) it was used in
#[derive(Debug, Serialize, Deserialize)]
pub struct ChipUsage {
    pub name: String,
    pub time: String,
    pub event: i32,
}

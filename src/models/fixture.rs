use serde::{Deserialize, Serialize};

/// A Premier League fixture (match) with scores and per-player stats
#[derive(Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub code: i32,
    /// The gameweek (event) this fixture belongs to `None` if unscheduled.
    pub event: Option<i32>,
    pub finished: bool,
    pub finished_provisional: bool,
    pub id: i32,
    pub kickoff_time: Option<String>,
    pub minutes: i32,
    pub provisional_start_time: bool,
    pub started: bool,
    /// Away team ID
    pub team_a: i32,
    pub team_a_score: Option<i32>,
    /// Home team ID
    pub team_h: i32,
    pub team_h_score: Option<i32>,
    pub stats: Vec<FixtureStat>,
    pub team_h_difficulty: i32,
    pub team_a_difficulty: i32,
    pub pulse_id: i32,
}

/// A single stat category (eg. goals_scored) for a fixture, split by home and away contributors.
#[derive(Debug, Serialize, Deserialize)]
pub struct FixtureStat {
    pub identifier: String,
    /// Away team contributions for this stat
    pub a: Vec<StatEntry>,
    /// Home team contributions for this stat
    pub h: Vec<StatEntry>,
}

/// A single element's (football player's) contribution to a fixture stat
#[derive(Debug, Serialize, Deserialize)]
pub struct StatEntry {
    pub value: i32,
    /// The element (football player) ID
    pub element: i32,
}

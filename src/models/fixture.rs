use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub code: i32,
    pub event: Option<i32>,
    pub finished: bool,
    pub finished_provisional: bool,
    pub id: i32,
    pub kickoff_time: Option<String>,
    pub minutes: i32,
    pub provisional_start_time: bool,
    pub started: bool,
    pub team_a: i32,
    pub team_a_score: Option<i32>,
    pub team_h: i32,
    pub team_h_score: Option<i32>,
    pub stats: Vec<FixtureStat>,
    pub team_h_difficulty: i32,
    pub team_a_difficulty: i32,
    pub pulse_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FixtureStat {
    pub identifier: String,
    pub a: Vec<StatEntry>,
    pub h: Vec<StatEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatEntry {
    pub value: i32,
    pub element: i32,
}

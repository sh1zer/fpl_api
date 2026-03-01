use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub code: u32,
    pub event: Option<u32>,
    pub finished: bool,
    pub finished_provisional: bool,
    pub id: u32,
    pub kickoff_time: Option<String>,
    pub minutes: u32,
    pub provisional_start_time: bool,
    pub started: bool,
    pub team_a: u32,
    pub team_a_score: Option<u32>,
    pub team_h: u32,
    pub team_h_score: Option<u32>,
    pub stats: Vec<FixtureStat>,
    pub team_h_difficulty: u32,
    pub team_a_difficulty: u32,
    pub pulse_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FixtureStat {
    pub identifier: String,
    pub a: Vec<StatEntry>,
    pub h: Vec<StatEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatEntry {
    pub value: u32,
    pub element: u32,
}

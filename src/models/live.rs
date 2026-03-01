use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveEvent {
    pub elements: Vec<LivePlayerStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LivePlayerStat {
    pub id: u32,
    pub stats: LiveStats,
    pub explain: Vec<PointsExplanation>,
    pub modified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStats {
    pub minutes: u32,
    pub goals_scored: u32,
    pub assists: u32,
    pub clean_sheets: u32,
    pub goals_conceded: u32,
    pub own_goals: u32,
    pub penalties_saved: u32,
    pub penalties_missed: u32,
    pub yellow_cards: u32,
    pub red_cards: u32,
    pub saves: u32,
    pub bonus: u32,
    pub bps: i32,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    pub ict_index: String,
    pub starts: u32,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub expected_goals_conceded: String,
    pub total_points: i32,
    pub in_dreamteam: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointsExplanation {
    pub fixture: u32,
    pub stats: Vec<ExplanationStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplanationStat {
    pub identifier: String,
    pub points: i32,
    pub value: u32,
}

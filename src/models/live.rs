use serde::{Deserialize, Serialize};

/// Live stats for all elements (football players) in a gameweek (event)
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveEvent {
    pub elements: Vec<LiveElementStat>,
}

/// Live stats and points breakdown for a single element (football player) in a gameweek
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveElementStat {
    pub id: i32,
    pub stats: LiveStats,
    pub explain: Vec<PointsExplanation>,
    pub modified: bool,
}

/// Real-time in-gameweek stats for a football player
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStats {
    pub minutes: i32,
    pub goals_scored: i32,
    pub assists: i32,
    pub clean_sheets: i32,
    pub goals_conceded: i32,
    pub own_goals: i32,
    pub penalties_saved: i32,
    pub penalties_missed: i32,
    pub yellow_cards: i32,
    pub red_cards: i32,
    pub saves: i32,
    pub bonus: i32,
    pub bps: i32,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    pub ict_index: String,
    pub starts: i32,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub expected_goals_conceded: String,
    pub total_points: i32,
    pub in_dreamteam: bool,
}

/// A breakdown of points earned by a player in a specific fixture
#[derive(Debug, Serialize, Deserialize)]
pub struct PointsExplanation {
    pub fixture: i32,
    pub stats: Vec<ExplanationStat>,
}

/// A single stat's contribution to a player's points tally in a fixture
#[derive(Debug, Serialize, Deserialize)]
pub struct ExplanationStat {
    pub identifier: String,
    pub points: i32,
    pub value: i32,
}

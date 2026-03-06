use serde::{Deserialize, Serialize};

/// Upcoming fixtures, past gameweek history, and season history for a single element (football player)
#[derive(Debug, Serialize, Deserialize)]
pub struct ElementSummary {
    pub fixtures: Vec<ElementFixture>,
    pub history: Vec<ElementHistory>,
    pub history_past: Vec<SeasonHistory>,
}

/// An upcoming or recent fixture from a specific element's (football player's) perspective
#[derive(Debug, Serialize, Deserialize)]
pub struct ElementFixture {
    pub id: i32,
    pub code: i32,
    /// Home team ID
    pub team_h: i32,
    pub team_h_score: Option<i32>,
    /// Away team ID
    pub team_a: i32,
    pub team_a_score: Option<i32>,
    pub event: i32,
    pub finished: bool,
    pub minutes: i32,
    pub provisional_start_time: bool,
    pub kickoff_time: Option<String>,
    pub event_name: String,
    pub is_home: bool,
    pub difficulty: i32,
}

/// An element's (football player's) stats for a single past fixture this season
#[derive(Debug, Serialize, Deserialize)]
pub struct ElementHistory {
    pub element: i32,
    pub fixture: i32,
    pub opponent_team: i32,
    pub total_points: i32,
    pub was_home: bool,
    pub kickoff_time: String,
    pub team_h_score: Option<i32>,
    pub team_a_score: Option<i32>,
    pub round: i32,
    pub modified: bool,
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
    pub clearances_blocks_interceptions: i32,
    pub recoveries: i32,
    pub tackles: i32,
    pub defensive_contribution: i32,
    pub starts: i32,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub expected_goals_conceded: String,
    /// Price in tenths of millions at the time of the fixture
    pub value: i32,
    pub transfers_balance: i32,
    pub selected: i32,
    pub transfers_in: i32,
    pub transfers_out: i32,
}

/// An element's (football player's) aggregated stats for a previous season
#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonHistory {
    pub season_name: String,
    pub element_code: i32,
    /// Price at the start of the season in tenths of millions
    pub start_cost: i32,
    /// Price at the end of the season in tenths of millions
    pub end_cost: i32,
    pub total_points: i32,
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
    pub clearances_blocks_interceptions: i32,
    pub recoveries: i32,
    pub tackles: i32,
    pub defensive_contribution: i32,
    pub starts: i32,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub expected_goals_conceded: String,
}

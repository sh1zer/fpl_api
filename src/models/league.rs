use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueStandings {
    pub new_entries: PaginatedResults<LeagueEntry>,
    pub last_updated_data: String,
    pub league: LeagueDetails,
    pub standings: PaginatedResults<StandingEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResults<T> {
    pub has_next: bool,
    pub page: u32,
    pub results: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueEntry {
    pub entry: u32,
    pub entry_name: String,
    pub joined_time: String,
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueDetails {
    pub id: u32,
    pub name: String,
    pub created: String,
    pub closed: bool,
    pub max_entries: Option<u32>,
    pub league_type: String,
    pub scoring: String,
    pub admin_entry: Option<u32>,
    pub start_event: u32,
    pub code_privacy: String,
    pub has_cup: bool,
    pub cup_league: Option<u32>,
    pub rank: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandingEntry {
    pub id: u32,
    pub event_total: i32,
    pub player_name: String,
    pub rank: u32,
    pub last_rank: u32,
    pub rank_sort: u32,
    pub total: i32,
    pub entry: u32,
    pub entry_name: String,
}

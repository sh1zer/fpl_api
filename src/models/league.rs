use serde::{Deserialize, Serialize};

/// Standings and entry list for a classic or head-to-head league
#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueStandings {
    pub new_entries: PaginatedResults<LeagueEntry>,
    pub last_updated_data: String,
    pub league: LeagueDetails,
    pub standings: PaginatedResults<StandingEntry>,
}

/// A paginated list of results with a `has_next` flag for further pages
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResults<T> {
    pub has_next: bool,
    pub page: i32,
    pub results: Vec<T>,
}

/// A new entry (manager) who recently joined the league
#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueEntry {
    pub entry: i32,
    pub entry_name: String,
    pub joined_time: String,
    pub player_name: String,
}

/// Metadata and configuration for a league
#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueDetails {
    pub id: i32,
    pub name: String,
    pub created: String,
    pub closed: bool,
    pub max_entries: Option<i32>,
    pub league_type: String,
    pub scoring: String,
    pub admin_entry: Option<i32>,
    pub start_event: i32,
    pub code_privacy: String,
    pub has_cup: bool,
    pub cup_league: Option<i32>,
    pub rank: Option<i32>,
}

/// A single entry's (manager's) position in a league standings table
#[derive(Debug, Serialize, Deserialize)]
pub struct StandingEntry {
    pub id: i32,
    pub event_total: i32,
    pub player_name: String,
    pub rank: i32,
    pub last_rank: i32,
    pub rank_sort: i32,
    pub total: i32,
    /// manager_id
    pub entry: i32,
    pub entry_name: String,
}

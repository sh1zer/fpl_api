use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An FPL manager (also called "entry" or "player" in the API) and their profile data
#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub id: i32,
    pub joined_time: String,
    /// The gameweek (event) the manager first entered FPL
    pub started_event: i32,
    pub favourite_team: Option<i32>,
    pub player_first_name: String,
    pub player_last_name: String,
    pub player_region_id: i32,
    pub player_region_name: String,
    pub player_region_iso_code_short: String,
    pub player_region_iso_code_long: String,
    pub summary_overall_points: Option<i32>,
    pub summary_overall_rank: Option<i32>,
    /// Points scored in the most recent gameweek (event)
    pub summary_event_points: Option<i32>,
    /// Rank in the most recent gameweek (event)
    pub summary_event_rank: Option<i32>,
    /// The most recent gameweek (event) ID the manager participated in
    pub current_event: Option<i32>,
    pub leagues: Leagues,
    /// The manager's team name
    pub name: String,
    pub name_change_blocked: bool,
    pub kit: Option<String>,
    pub last_deadline_bank: Option<i32>,
    pub last_deadline_value: Option<i32>,
    pub last_deadline_total_transfers: Option<i32>,
    /// IDs of gameweeks (events) the manager has entered
    pub entered_events: Vec<i32>,
    pub years_active: i32,
    pub club_badge_src: Option<String>,
}

/// All leagues a manager belongs to, plus their cup status
#[derive(Debug, Serialize, Deserialize)]
pub struct Leagues {
    pub classic: Vec<LeagueEntry>,
    pub h2h: Vec<LeagueEntry>,
    pub cup: Cup,
    pub cup_matches: Vec<CupMatch>,
    pub event: Vec<Value>,
}

/// The manager's cup bracket and status
#[derive(Debug, Serialize, Deserialize)]
pub struct Cup {
    pub matches: Vec<CupMatch>,
    pub status: CupStatus,
    pub cup_league: Option<i32>,
}

/// A league a manager is enrolled in, including their current rank
#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueEntry {
    pub id: i32,
    pub name: String,
    pub short_name: Option<String>,
    pub created: String,
    pub closed: bool,
    pub rank: Option<i32>,
    pub max_entries: Option<i32>,
    pub league_type: String,
    pub scoring: String,
    /// The entry (manager) ID of the league admin
    pub admin_entry: Option<i32>,
    pub start_event: i32,
    pub entry_can_leave: bool,
    pub entry_can_admin: bool,
    pub entry_can_invite: bool,
    pub has_cup: bool,
    pub cup_league: Option<i32>,
    pub cup_qualified: Option<bool>,
    pub entry_rank: i32,
    pub entry_last_rank: i32,
}

/// A manager's cup qualification status and progress
#[derive(Debug, Serialize, Deserialize)]
pub struct CupStatus {
    pub qualification_event: Option<i32>,
    pub qualification_numbers: Option<i32>,
    pub qualification_rank: Option<i32>,
    pub qualification_state: Option<String>,
}

/// A single head-to-head cup match between two entries (managers)
#[derive(Debug, Serialize, Deserialize)]
pub struct CupMatch {
    pub id: i32,
    /// manager_id
    pub entry_1_entry: i32,
    pub entry_1_name: String,
    pub entry_1_player_name: String,
    pub entry_1_points: i32,
    pub entry_1_win: i32,
    pub entry_1_draw: i32,
    pub entry_1_loss: i32,
    pub entry_1_total: i32,
    /// manager_id
    pub entry_2_entry: i32,
    pub entry_2_name: String,
    pub entry_2_player_name: String,
    pub entry_2_points: i32,
    pub entry_2_win: i32,
    pub entry_2_draw: i32,
    pub entry_2_loss: i32,
    pub entry_2_total: i32,
    pub is_knockout: bool,
    pub is_bye: bool,
    pub league: i32,
    /// The winning entry (manager) ID, if the match is complete
    pub winner: Option<i32>,
    pub seed_value: Option<i32>,
    pub tiebreak: Option<String>,
    pub event: i32,
    pub knockout_name: Option<String>,
}

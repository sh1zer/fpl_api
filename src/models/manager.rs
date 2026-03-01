use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub id: i32,
    pub joined_time: String,
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
    pub summary_event_points: Option<i32>,
    pub summary_event_rank: Option<i32>,
    pub current_event: Option<i32>,
    pub leagues: Leagues,
    pub name: String,
    pub name_change_blocked: bool,
    pub kit: Option<String>,
    pub last_deadline_bank: Option<i32>,
    pub last_deadline_value: Option<i32>,
    pub last_deadline_total_transfers: Option<i32>,
    pub entered_events: Vec<i32>,
    pub years_active: Vec<i32>,
    pub club_badge_src: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leagues {
    pub classic: Vec<LeagueEntry>,
    pub h2h: Vec<LeagueEntry>,
    pub cup: CupStatus,
    pub cup_matches: Vec<CupMatch>,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CupStatus {
    pub qualification_event: Option<i32>,
    pub qualification_numbers: Option<i32>,
    pub qualification_rank: Option<i32>,
    pub qualification_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CupMatch {
    pub id: i32,
    pub entry_1_entry: i32,
    pub entry_1_name: String,
    pub entry_1_player_name: String,
    pub entry_1_points: i32,
    pub entry_1_win: i32,
    pub entry_1_draw: i32,
    pub entry_1_loss: i32,
    pub entry_1_total: i32,
    pub entry_2_entry: i32,
    pub entry_2_name: String,
    pub entry_2_player_name: String,
    pub entry_2_points: i32,
    pub entry_2_win: i32,
    pub entry_2_draw: i32,
    pub entry_2_loss: i32,
    pub entry_2_total: i32,
    pub is_bye: bool,
    pub knockout_type: String,
    pub event: i32,
    pub winner: i32,
}

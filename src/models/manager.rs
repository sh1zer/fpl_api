use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub id: u32,
    pub joined_time: String,
    pub started_event: u32,
    pub favourite_team: Option<u32>,
    pub player_first_name: String,
    pub player_last_name: String,
    pub player_region_id: u32,
    pub player_region_name: String,
    pub player_region_iso_code_short: String,
    pub player_region_iso_code_long: String,
    pub summary_overall_points: Option<i32>,
    pub summary_overall_rank: Option<u32>,
    pub summary_event_points: Option<i32>,
    pub summary_event_rank: Option<u32>,
    pub current_event: Option<u32>,
    pub leagues: Leagues,
    pub name: String,
    pub name_change_blocked: bool,
    pub kit: Option<String>,
    pub last_deadline_bank: Option<u32>,
    pub last_deadline_value: Option<u32>,
    pub last_deadline_total_transfers: Option<u32>,
    pub entered_events: Vec<u32>,
    pub years_active: Vec<u32>,
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
    pub id: u32,
    pub name: String,
    pub short_name: Option<String>,
    pub created: String,
    pub closed: bool,
    pub rank: Option<u32>,
    pub max_entries: Option<u32>,
    pub league_type: String,
    pub scoring: String,
    pub admin_entry: Option<u32>,
    pub start_event: u32,
    pub entry_can_leave: bool,
    pub entry_can_admin: bool,
    pub entry_can_invite: bool,
    pub has_cup: bool,
    pub cup_league: Option<u32>,
    pub cup_qualified: Option<bool>,
    pub entry_rank: u32,
    pub entry_last_rank: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CupStatus {
    pub qualification_event: Option<u32>,
    pub qualification_numbers: Option<u32>,
    pub qualification_rank: Option<u32>,
    pub qualification_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CupMatch {
    pub id: u32,
    pub entry_1_entry: u32,
    pub entry_1_name: String,
    pub entry_1_player_name: String,
    pub entry_1_points: u32,
    pub entry_1_win: u32,
    pub entry_1_draw: u32,
    pub entry_1_loss: u32,
    pub entry_1_total: u32,
    pub entry_2_entry: u32,
    pub entry_2_name: String,
    pub entry_2_player_name: String,
    pub entry_2_points: u32,
    pub entry_2_win: u32,
    pub entry_2_draw: u32,
    pub entry_2_loss: u32,
    pub entry_2_total: u32,
    pub is_bye: bool,
    pub knockout_type: String,
    pub event: u32,
    pub winner: u32,
}

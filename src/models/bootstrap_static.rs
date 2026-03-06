use serde::{Deserialize, Serialize};

/// Top-level response from the bootstrap-static endpoint Contains all global game data
#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapStatic {
    pub events: Vec<Event>,
    pub game_settings: GameSettings,
    pub phases: Vec<Phase>,
    pub teams: Vec<Team>,
    pub total_players: i32,
    pub elements: Vec<Element>,
    pub element_stats: Vec<ElementStat>,
    pub element_types: Vec<ElementType>,
    pub chips: Vec<Chip>,
    pub game_config: Option<GameConfig>,
}

/// A gameweek (also called an "event") in the FPL season
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub deadline_time: String,
    pub release_time: Option<String>,
    pub average_entry_score: i32,
    pub finished: bool,
    pub data_checked: bool,
    pub highest_scoring_entry: Option<i32>,
    pub deadline_time_epoch: u64,
    pub deadline_time_game_offset: i32,
    pub highest_score: Option<i32>,
    pub is_previous: bool,
    pub is_current: bool,
    pub is_next: bool,
    pub cup_leagues_created: bool,
    pub h2h_ko_matches_created: bool,
    pub can_enter: bool,
    pub can_manage: bool,
    pub released: bool,
    pub ranked_count: i32,
    pub chip_plays: Vec<ChipPlay>,
    pub most_selected: Option<i32>,
    pub most_transferred_in: Option<i32>,
    pub top_element: Option<i32>,
    pub top_element_info: Option<TopElementInfo>,
    pub transfers_made: i32,
    pub most_captained: Option<i32>,
    pub most_vice_captained: Option<i32>,
}

/// How many times a chip was played in a given gameweek
#[derive(Debug, Serialize, Deserialize)]
pub struct ChipPlay {
    pub chip_name: String,
    pub num_played: i32,
}

/// The top scoring element (football player) and their points for a gameweek
#[derive(Debug, Serialize, Deserialize)]
pub struct TopElementInfo {
    pub id: i32,
    pub points: i32,
}

/// A Premier League team and its FPL-related stats
#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub code: i32,
    pub draw: i32,
    pub form: Option<String>,
    pub id: i32,
    pub loss: i32,
    pub name: String,
    pub played: i32,
    pub points: i32,
    pub position: i32,
    pub short_name: String,
    pub strength: i32,
    pub team_division: Option<String>,
    pub unavailable: bool,
    pub win: i32,
    pub strength_overall_home: i32,
    pub strength_overall_away: i32,
    pub strength_attack_home: i32,
    pub strength_attack_away: i32,
    pub strength_defence_home: i32,
    pub strength_defence_away: i32,
    pub pulse_id: i32,
}

/// A football player (referred to as "element" in the FPL API)
#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    pub chance_of_playing_next_round: Option<i32>,
    pub chance_of_playing_this_round: Option<i32>,
    pub code: i32,
    pub cost_change_event: i32,
    pub cost_change_event_fall: i32,
    pub cost_change_start: i32,
    pub cost_change_start_fall: i32,
    pub dreamteam_count: i32,
    /// Position type ID (1=GKP, 2=DEF, 3=MID, 4=FWD)
    pub element_type: i32,
    pub ep_next: String,
    pub ep_this: String,
    pub event_points: i32,
    pub first_name: String,
    pub form: String,
    pub id: i32,
    pub in_dreamteam: bool,
    pub news: String,
    pub news_added: Option<String>,
    /// Current price in tenths of millions (eg. 65 = £6.5m)
    pub now_cost: i32,
    pub photo: String,
    pub points_per_game: String,
    pub second_name: String,
    pub selected_by_percent: String,
    pub special: bool,
    pub squad_number: Option<i32>,
    pub status: String,
    pub team: i32,
    pub team_code: i32,
    pub total_points: i32,
    pub transfers_in: i32,
    pub transfers_in_event: i32,
    pub transfers_out: i32,
    pub transfers_out_event: i32,
    pub value_form: String,
    pub value_season: String,
    pub web_name: String,
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
    pub influence_rank: i32,
    pub influence_rank_type: i32,
    pub creativity_rank: i32,
    pub creativity_rank_type: i32,
    pub threat_rank: i32,
    pub threat_rank_type: i32,
    pub ict_index_rank: i32,
    pub ict_index_rank_type: i32,
    pub corners_and_indirect_freekicks_order: Option<i32>,
    pub corners_and_indirect_freekicks_text: String,
    pub direct_freekicks_order: Option<i32>,
    pub direct_freekicks_text: String,
    pub penalties_order: Option<i32>,
    pub penalties_text: String,
    pub expected_goals_per_90: f32,
    pub saves_per_90: f32,
    pub expected_assists_per_90: f32,
    pub expected_goal_involvements_per_90: f32,
    pub expected_goals_conceded_per_90: f32,
    pub goals_conceded_per_90: f32,
    pub clean_sheets_per_90: f32,
    pub starts_per_90: f32,
    pub now_cost_rank: i32,
    pub now_cost_rank_type: i32,
    pub form_rank: i32,
    pub form_rank_type: i32,
    pub points_per_game_rank: i32,
    pub points_per_game_rank_type: i32,
    pub selected_rank: i32,
    pub selected_rank_type: i32,
    pub birth_date: Option<String>,
    pub region: Option<i32>,
    pub team_join_date: Option<String>,
    pub known_name: Option<String>,
    pub removed: bool,
    pub opta_code: String,
    pub has_temporary_code: bool,
    pub can_select: bool,
    pub can_transact: bool,
    pub tackles: i32,
    pub recoveries: i32,
    pub clearances_blocks_interceptions: i32,
    pub defensive_contribution: i32,
    pub defensive_contribution_per_90: f32,
    pub scout_risks: Vec<ScoutRisk>,
}

/// An injury or availability risk note for a football player
#[derive(Debug, Serialize, Deserialize)]
pub struct ScoutRisk {
    pub property: String,
    pub notes: String,
    pub gameweek: Option<i32>,
    pub url: Option<String>,
}

/// Global game rules and configuration limits
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub league_join_private_max: i32,
    pub league_join_public_max: i32,
    pub league_max_size_public_classic: i32,
    pub league_max_size_public_h2h: i32,
    pub league_max_size_private_h2h: i32,
    pub league_max_ko_rounds_private_h2h: i32,
    pub league_prefix_public: String,
    pub league_points_h2h_win: i32,
    pub league_points_h2h_lose: i32,
    pub league_points_h2h_draw: i32,
    pub league_ko_first_instead_of_random: bool,
    pub cup_start_event_id: Option<i32>,
    pub cup_stop_event_id: Option<i32>,
    pub cup_qualifying_method: Option<String>,
    pub cup_type: Option<String>,
    pub featured_entries: Vec<i32>,
    pub element_sell_at_purchase_price: bool,
    pub percentile_ranks: Vec<i32>,
    pub underdog_differential: i32,
    pub squad_squadplay: i32,
    pub squad_squadsize: i32,
    pub squad_special_min: Option<i32>,
    pub squad_special_max: Option<i32>,
    pub squad_team_limit: i32,
    pub squad_total_spend: i32,
    pub ui_currency_multiplier: i32,
    pub ui_use_special_shirts: bool,
    pub ui_special_shirt_exclusions: Vec<i32>,
    pub stats_form_days: i32,
    pub sys_vice_captain_enabled: bool,
    pub transfers_cap: i32,
    pub transfers_sell_on_fee: f32,
    pub max_extra_free_transfers: i32,
    pub league_h2h_tiebreak_stats: Vec<String>,
    pub timezone: Option<String>,
}

/// A named phase of the season (eg. "Overall", "Month 1"), spanning a range of gameweeks.
#[derive(Debug, Serialize, Deserialize)]
pub struct Phase {
    pub id: i32,
    pub name: String,
    pub start_event: i32,
    pub stop_event: i32,
    pub highest_score: Option<i32>,
}

/// Metadata for a single element (football player) stat, such as "Goals Scored"
#[derive(Debug, Serialize, Deserialize)]
pub struct ElementStat {
    pub label: String,
    pub name: String,
}

/// A position type (GKP, DEF, MID, FWD) and its squad selection rules
#[derive(Debug, Serialize, Deserialize)]
pub struct ElementType {
    pub id: i32,
    pub plural_name: String,
    pub plural_name_short: String,
    pub singular_name: String,
    pub singular_name_short: String,
    pub squad_select: i32,
    pub squad_min_select: Option<i32>,
    pub squad_max_select: Option<i32>,
    pub squad_min_play: i32,
    pub squad_max_play: i32,
    pub ui_shirt_specific: bool,
    pub sub_positions_locked: Vec<i32>,
    pub element_count: i32,
}

/// An FPL chip (eg. Wildcard, Triple Captain) and the gameweeks it is available in
#[derive(Debug, Serialize, Deserialize)]
pub struct Chip {
    pub id: i32,
    pub name: String,
    pub number: i32,
    pub start_event: i32,
    pub stop_event: i32,
    pub chip_type: String,
}

/// Combined game configuration including settings, rules and scoring
#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub settings: GameConfigSettings,
    pub rules: GameSettings,
    pub scoring: ScoringRules,
}

/// Basic game configuration settings such as timezone
#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfigSettings {
    pub entry_per_event: bool,
    pub timezone: String,
}

/// Points awarded per position for a stat category
#[derive(Debug, Serialize, Deserialize)]
pub struct PositionScore {
    #[serde(rename = "DEF")]
    pub def: i32,
    #[serde(rename = "FWD")]
    pub fwd: i32,
    #[serde(rename = "GKP")]
    pub gkp: i32,
    #[serde(rename = "MID")]
    pub mid: i32,
}

/// The full FPL points scoring rules for all stats and positions
#[derive(Debug, Serialize, Deserialize)]
pub struct ScoringRules {
    pub long_play: i32,
    pub short_play: i32,
    pub goals_conceded: PositionScore,
    pub saves: i32,
    pub goals_scored: PositionScore,
    pub assists: i32,
    pub clean_sheets: PositionScore,
    pub penalties_saved: i32,
    pub penalties_missed: i32,
    pub yellow_cards: i32,
    pub red_cards: i32,
    pub own_goals: i32,
    pub bonus: i32,
    pub bps: i32,
    pub influence: i32,
    pub creativity: i32,
    pub threat: i32,
    pub ict_index: i32,
    pub special_multiplier: i32,
    pub tackles: i32,
    pub clearances_blocks_interceptions: i32,
    pub recoveries: i32,
    pub defensive_contribution: PositionScore,
    pub starts: i32,
    pub mng_goals_scored: PositionScore,
    pub mng_clean_sheets: PositionScore,
    pub mng_win: PositionScore,
    pub mng_draw: PositionScore,
    pub mng_loss: i32,
    pub mng_underdog_win: PositionScore,
    pub mng_underdog_draw: PositionScore,
    pub expected_assists: i32,
    pub expected_goal_involvements: i32,
    pub expected_goals_conceded: i32,
    pub expected_goals: i32,
}

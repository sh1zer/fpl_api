use serde::{Deserialize, Serialize};

/// A manager's squad picks, chip, and gameweek history for a specific gameweek (event)
#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerPicks {
    pub active_chip: Option<String>,
    pub automatic_subs: Vec<AutomaticSub>,
    pub entry_history: EntryHistory,
    pub picks: Vec<Pick>,
}

/// A single element (football player) picked in a manager's squad for a gameweek
#[derive(Debug, Serialize, Deserialize)]
pub struct Pick {
    /// The element (football player) ID
    pub element: i32,
    /// Position in the squad (1-11 starting, 12-15 bench)
    pub position: i32,
    /// Points multiplier: 1 for normal, 2 for captain, 3 for triple captain
    pub multiplier: i32,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub element_type: i32,
}

/// An automatic substitution made during a gameweek due to a player not playing
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticSub {
    /// The entry (manager) ID
    pub entry: i32,
    /// The element (football player) ID who came on
    pub element_in: i32,
    /// The element (football player) ID who was subbed off
    pub element_out: i32,
    /// The gameweek (event) the substitution occurred in
    pub event: i32,
}

/// A manager's points, rank, and squad value snapshot for a single gameweek (event)
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryHistory {
    /// The gameweek (event) ID
    pub event: i32,
    pub points: i32,
    pub total_points: i32,
    pub rank: Option<i32>,
    pub rank_sort: Option<i32>,
    pub overall_rank: Option<i32>,
    /// Remaining transfer budget in tenths of millions (eg. 10 = £1.0m).
    pub bank: i32,
    /// Total squad value + bank in tenths of millions
    pub value: i32,
    pub event_transfers: i32,
    pub event_transfers_cost: i32,
    pub points_on_bench: i32,
    pub percentile_rank: Option<i32>,
}

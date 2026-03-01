use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerPicks {
    pub active_chip: Option<String>,
    pub automatic_subs: Vec<AutomaticSub>,
    pub entry_history: EntryHistory,
    pub picks: Vec<Pick>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pick {
    pub element: i32,
    pub position: i32,
    pub multiplier: i32,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub element_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticSub {
    pub entry: i32,
    pub element_in: i32,
    pub element_out: i32,
    pub event: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryHistory {
    pub event: i32,
    pub points: i32,
    pub total_points: i32,
    pub rank: Option<i32>,
    pub rank_sort: Option<i32>,
    pub overall_rank: Option<i32>,
    pub bank: i32,
    pub value: i32,
    pub event_transfers: i32,
    pub event_transfers_cost: i32,
    pub points_on_bench: i32,
    pub percentile_rank: Option<i32>,
}

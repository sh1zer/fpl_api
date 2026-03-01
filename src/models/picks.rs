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
    pub element: u32,
    pub position: u32,
    pub multiplier: u32,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub element_type: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticSub {
    pub entry: u32,
    pub element_in: u32,
    pub element_out: u32,
    pub event: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryHistory {
    pub event: u32,
    pub points: i32,
    pub total_points: i32,
    pub rank: Option<u32>,
    pub rank_sort: Option<u32>,
    pub overall_rank: Option<u32>,
    pub bank: u32,
    pub value: u32,
    pub event_transfers: u32,
    pub event_transfers_cost: u32,
    pub points_on_bench: u32,
    pub percentile_rank: Option<u32>,
}

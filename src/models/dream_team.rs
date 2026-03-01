use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DreamTeam {
    pub top_player: TopPlayer,
    pub team: Vec<DreamTeamPlayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopPlayer {
    pub id: u32,
    pub points: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DreamTeamPlayer {
    pub element: u32,
    pub points: u32,
    pub position: u32,
}

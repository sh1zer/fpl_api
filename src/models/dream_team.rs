use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DreamTeam {
    pub top_player: TopPlayer,
    pub team: Vec<DreamTeamPlayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopPlayer {
    pub id: i32,
    pub points: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DreamTeamPlayer {
    pub element: i32,
    pub points: i32,
    pub position: i32,
}

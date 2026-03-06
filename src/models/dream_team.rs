use serde::{Deserialize, Serialize};

/// The FPL dream team for a gameweek — the highest scoring XI of elements (football players)
#[derive(Debug, Serialize, Deserialize)]
pub struct DreamTeam {
    pub top_player: TopPlayer,
    pub team: Vec<DreamTeamPlayer>,
}

/// The single highest scoring element (football player) in the dream team
#[derive(Debug, Serialize, Deserialize)]
pub struct TopPlayer {
    pub id: i32,
    pub points: i32,
}

/// An element (football player) selected in the dream team for a gameweek
#[derive(Debug, Serialize, Deserialize)]
pub struct DreamTeamPlayer {
    pub element: i32,
    pub points: i32,
    pub position: i32,
}

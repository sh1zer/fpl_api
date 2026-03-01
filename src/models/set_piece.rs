use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceNotes {
    pub last_updated: String,
    pub teams: Vec<SetPieceTeam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceTeam {
    pub id: u32,
    pub notes: Vec<SetPieceNote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceNote {
    pub info: String,
    pub source: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceNotes {
    pub last_updated: String,
    pub teams: Vec<SetPieceTeam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceTeam {
    pub id: i32,
    pub notes: Vec<SetPieceNote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceNote {
    pub external_link: bool,
    pub info_message: String,
    pub source_link: String,
}

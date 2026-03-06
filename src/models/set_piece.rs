use serde::{Deserialize, Serialize};

/// Set piece taking notes for all teams, indicating who takes corners, free kicks, and penalties
#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceNotes {
    pub last_updated: String,
    pub teams: Vec<SetPieceTeam>,
}

/// Set piece notes for a single Premier League team
#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceTeam {
    pub id: i32,
    pub notes: Vec<SetPieceNote>,
}

/// A single set piece note, describing who takes a specific type of set piece
#[derive(Debug, Serialize, Deserialize)]
pub struct SetPieceNote {
    pub external_link: bool,
    pub info_message: String,
    pub source_link: String,
}

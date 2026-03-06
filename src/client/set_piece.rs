use crate::client::FplApiClient;
use crate::models::set_piece::SetPieceNotes;
use anyhow::Result;

impl FplApiClient {
    /// gets set piece notes of all teams
    pub async fn get_set_piece(&self) -> Result<SetPieceNotes> {
        self.gets("team/set-piece-notes").await
    }
}

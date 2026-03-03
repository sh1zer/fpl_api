use crate::LeagueStandings;
use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use crate::models::set_piece::SetPieceNotes;
use anyhow::Result;
use std::collections::HashMap;

impl FplApiClient {
    /// gets set piece notes of all teams
    pub async fn get_set_piece(&self) -> Result<SetPieceNotes> {
        self.gets("team/set-piece-notes").await
    }
}

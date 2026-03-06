use crate::client::FplApiClient;
use crate::models::dream_team::DreamTeam;
use anyhow::Result;

impl FplApiClient {
    /// gets dream team for given gameweek
    pub async fn get_dream_team(&self, event_id: i32) -> Result<DreamTeam> {
        self.gets(format!("dream-team/{}", event_id).as_str()).await
    }
}

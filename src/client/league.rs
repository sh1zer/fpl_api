use crate::LeagueStandings;
use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use anyhow::Result;
use std::collections::HashMap;

impl FplApiClient {
    /// gets league standings for a give league id
    pub async fn get_league_standings(&self, league_id: i32) -> Result<LeagueStandings> {
        self.gets(format!("leagues-classic/{}/standings", league_id).as_str())
            .await
    }
}

use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use anyhow::Result;

impl FplApiClient {
    /// gets fixtures for either the whole season,
    /// allows for filtering by team and gameweek
    pub async fn get_fixtures(
        &self,
        gameweek: Option<i32>,
        team: Option<i32>,
    ) -> Result<Vec<Fixture>> {
        let mut query = vec![];
        if let Some(gw) = gameweek {
            query.push(("event", gw));
        }
        if let Some(id) = team {
            query.push(("team", id));
        }

        let fixtures: Vec<Fixture> = self.get("fixtures", Some(query)).await?;

        Ok(fixtures)
    }
}

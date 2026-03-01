use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use anyhow::Result;
use std::collections::HashMap;

impl FplApiClient {
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

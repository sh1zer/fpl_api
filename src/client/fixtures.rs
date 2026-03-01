use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use anyhow::Result;
use std::collections::HashMap;

impl FplApiClient {
    pub async fn get_fixtures(&self, gameweek: i32) -> Result<Vec<Fixture>> {
        let response: Vec<Fixture> = self.get("fixtures", Some([("event", gameweek)])).await?;

        Ok(response)
    }
}

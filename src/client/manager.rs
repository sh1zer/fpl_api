use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use crate::{Manager, ManagerPicks};
use anyhow::Result;
use std::collections::HashMap;

impl FplApiClient {
    /// gets data for a given manager
    pub async fn get_manager(&self, manager_id: i32) -> Result<Manager> {
        let manager: Manager = self.gets(format!("entry/{}", manager_id).as_str()).await?;

        Ok(manager)
    }

    /// gets picks for a given manager on a given gameweek
    pub async fn get_manager_picks(&self, manager_id: i32, event_id: i32) -> Result<ManagerPicks> {
        let picks: ManagerPicks = self
            .gets(format!("entry/{}/event/{}/picks", manager_id, event_id).as_str())
            .await?;

        Ok(picks)
    }
}

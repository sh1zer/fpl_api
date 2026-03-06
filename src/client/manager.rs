use crate::client::FplApiClient;
use crate::{Manager, ManagerHistory, ManagerPicks};
use anyhow::Result;

impl FplApiClient {
    /// gets data for a given manager
    pub async fn get_manager(&self, manager_id: i32) -> Result<Manager> {
        let manager: Manager = self.gets(format!("entry/{}", manager_id).as_str()).await?;

        Ok(manager)
    }

    pub async fn get_manager_history(&self, manager_id: i32) -> Result<ManagerHistory> {
        let history: ManagerHistory = self
            .gets(format!("entry/{}/history", manager_id).as_str())
            .await?;
        Ok(history)
    }

    /// gets picks for a given manager on a given gameweek
    pub async fn get_manager_picks(&self, manager_id: i32, event_id: i32) -> Result<ManagerPicks> {
        let picks: ManagerPicks = self
            .gets(format!("entry/{}/event/{}/picks", manager_id, event_id).as_str())
            .await?;

        Ok(picks)
    }
}

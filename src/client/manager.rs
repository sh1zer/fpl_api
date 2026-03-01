use crate::Manager;
use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use anyhow::Result;
use std::collections::HashMap;

impl FplApiClient {
    pub async fn get_manager(&self, manager_id: i32) -> Result<Manager> {
        let manager: Manager = self.gets(format!("entry/{}", manager_id).as_str()).await?;

        Ok(manager)
    }
}

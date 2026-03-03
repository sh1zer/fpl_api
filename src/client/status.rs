use crate::client::FplApiClient;
use crate::models::status::EventStatus;
use anyhow::Result;

impl FplApiClient {
    /// gets the evene-status endpoint
    pub async fn get_event_status(&self) -> Result<EventStatus> {
        let status: EventStatus = self.gets("event-status/").await?;
        Ok(status)
    }
}

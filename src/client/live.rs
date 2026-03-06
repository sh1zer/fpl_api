use crate::LiveEvent;
use crate::client::FplApiClient;
use crate::models::live::LiveElementStat;
use anyhow::Result;
use std::collections::HashSet;

impl FplApiClient {
    /// returns all live data for the given event_id
    ///
    /// for current week info you first need to consult the bootstrap -> events -> is_current
    /// values to find the event_id
    pub async fn get_live(&self, event_id: i32) -> Result<LiveEvent> {
        let response: LiveEvent = self
            .gets(format!("event/{}/live/", event_id).as_str())
            .await?;

        Ok(response)
    }

    /// returns all live data for given set of players (iterator of element_ids)
    pub async fn get_live_element_stats<I>(
        &self,
        event_id: i32,
        element_ids: I,
    ) -> Result<Vec<LiveElementStat>>
    where
        I: IntoIterator<Item = i32>,
    {
        let response: LiveEvent = self
            .gets(format!("event/{}/live/", event_id).as_str())
            .await?;

        let id_set: HashSet<i32> = element_ids.into_iter().collect();

        let players: Vec<LiveElementStat> = response
            .elements
            .into_iter()
            .filter(|p| id_set.contains(&p.id))
            .collect();

        Ok(players)
    }
}

use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use crate::models::live::LivePlayerStat;
use crate::{LiveEvent, Manager, ManagerPicks};
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

impl FplApiClient {
    pub async fn get_live(&self, event_id: i32) -> Result<LiveEvent> {
        let response: LiveEvent = self
            .gets(format!("event/{}/live/", event_id).as_str())
            .await?;

        Ok(response)
    }

    pub async fn get_live_player_stats<I>(
        &self,
        event_id: i32,
        player_ids: I,
    ) -> Result<Vec<LivePlayerStat>>
    where
        I: IntoIterator<Item = i32>,
    {
        let response: LiveEvent = self
            .gets(format!("event/{}/live/", event_id).as_str())
            .await?;

        let id_set: HashSet<i32> = player_ids.into_iter().collect();

        let players: Vec<LivePlayerStat> = response
            .elements
            .into_iter()
            .filter(|p| id_set.contains(&p.id))
            .collect();

        Ok(players)
    }
}

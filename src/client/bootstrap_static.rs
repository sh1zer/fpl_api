use crate::client::{FplApiClient, bootstrap_static};
use crate::models::bootstrap_static::{Element, Event, Team};
use crate::models::element_summary::ElementSummary;
use crate::models::fixture::Fixture;
use crate::{BootstrapStatic, Manager};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::collections::HashSet;

impl FplApiClient {
    /// returns the BootstrapStatic struct, containing all static data such as individual player and team data
    pub async fn get_bootstrap(&self) -> Result<BootstrapStatic> {
        let all: BootstrapStatic = self.gets("bootstrap_static").await?;

        Ok(all)
    }

    pub async fn get_events(&self) -> Result<Vec<Event>> {
        let events: Vec<Event> = self.gets("events/").await?;
        Ok(events)
    }

    /// retrieves info for a set of teams (iterator of team_ids), not recommended to call in loops as each one makes an
    /// api call and then has to iterate through the entire 1000 element array of player data
    ///
    /// if possible store the all the bootstrap_static data somewhere locally to avoid excessive calls of this
    /// endpoint
    pub async fn get_team_info<I>(&self, team_id: I) -> Result<Vec<Team>>
    where
        I: IntoIterator<Item = i32>,
    {
        let all: BootstrapStatic = self.get_bootstrap().await?;

        let id_set: HashSet<i32> = team_id.into_iter().collect();

        let teams: Vec<Team> = all
            .teams
            .into_iter()
            .filter(|t| id_set.contains(&t.id))
            .collect();

        Ok(teams)
    }
}

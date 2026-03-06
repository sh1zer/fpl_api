use crate::BootstrapStatic;
use crate::client::FplApiClient;
use crate::models::bootstrap_static::Element;
use crate::models::element_summary::ElementSummary;
use anyhow::Result;
use std::collections::HashSet;

impl FplApiClient {
    /// returns upcoming as well as previous fixtures in the season
    ///
    /// also includes previous season summaries
    pub async fn get_element_matches(&self, element_id: i32) -> Result<ElementSummary> {
        let summary: ElementSummary = self
            .gets(format!("element-summary/{}/", element_id).as_str())
            .await?;

        Ok(summary)
    }

    /// retrieves info for a set of players (iterator of element_ids), not recommended to call in loops as each one makes an
    /// api call and then has to iterate through the entire 1000 element array of player data
    ///
    /// if possible store the all the bootstrap_static data somewhere locally to avoid excessive calls of this
    /// endpoint
    pub async fn get_element_info<I>(&self, element_ids: I) -> Result<Vec<Element>>
    where
        I: IntoIterator<Item = i32>,
    {
        let all: BootstrapStatic = self.gets("bootstrap_static").await?;
        let id_set: HashSet<i32> = element_ids.into_iter().collect();

        let elements: Vec<Element> = all
            .elements
            .into_iter()
            .filter(|e| id_set.contains(&e.id))
            .collect();

        Ok(elements)
    }
}

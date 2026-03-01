use crate::client::FplApiClient;
use crate::models::bootstrap_static::Element;
use crate::models::element_summary::ElementSummary;
use crate::models::fixture::Fixture;
use crate::{BootstrapStatic, Manager};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

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

    /// retrieves a singular players info, not recommended to call in loops as each one makes an
    /// api call and then has to iterate through the entire 1000 element array of player data
    ///
    /// if possible store the all the bootstrap_static data somewhere locally to avoid excessive calls of this
    /// endpoint
    pub async fn get_element_info(&self, element_id: i32) -> Result<Element> {
        let all: BootstrapStatic = self
            .gets(format!("element-summary/{}/", element_id).as_str())
            .await?;
        let element: Element = match all.elements.into_iter().find(|e| e.id == element_id) {
            Some(element) => element,
            _ => {
                return Err(anyhow!("Couldnt find player with given element_id"));
            }
        };

        Ok(element)
    }
}

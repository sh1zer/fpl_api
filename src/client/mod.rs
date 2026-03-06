use anyhow::Result;
use reqwest::{Client, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::time::Duration;

pub mod bootstrap_static;
pub mod dream_team;
pub mod fixtures;
pub mod league;
pub mod live;
pub mod manager;
pub mod player;
pub mod set_piece;
pub mod status;

/// the client for all things request related, always go through this for anything
pub struct FplApiClient {
    pub(crate) client: Client,
    pub(crate) base_url: String,
}

impl FplApiClient {
    /// creates a new client using default config
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .pool_idle_timeout(Duration::from_secs(300))
            .pool_max_idle_per_host(2)
            .timeout(Duration::from_secs(5))
            .user_agent("fpl-api-rust-client/0.1.0")
            .build()?;

        let base_url = "https://fantasy.premierleague.com/api".to_string();
        Ok(FplApiClient { client, base_url })
    }

    /// creates a new client using provided builder config
    pub fn from_builder(builder: reqwest::ClientBuilder) -> Result<Self> {
        let client = builder.build()?;

        let base_url = "https://fantasy.premierleague.com/api".to_string();
        Ok(FplApiClient { client, base_url })
    }

    /// internal function for any GET request to the api
    pub(crate) async fn get<T, Q>(&self, endpoint: impl Into<String>, query: Option<Q>) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint.into());

        let mut request = self.client.get(url);

        if let Some(query) = query {
            request = request.query(&query);
        }

        let response: Response = request.send().await?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP Error: {}", response.status()));
        }

        Ok(response.json::<T>().await?)
    }
    /// internal function for GET requests without additional parameters, basically just a get
    /// wrapper for simplicity
    pub(crate) async fn gets<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(endpoint, None::<()>).await
    }
}

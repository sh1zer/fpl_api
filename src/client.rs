#![allow(unused)]

use anyhow::Result;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

struct FplApiClient {
    client: Client,
    base_url: String,
}

impl FplApiClient {
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

    pub fn from_builder(builder: reqwest::ClientBuilder) -> Result<Self> {
        let client = builder.build()?;

        let base_url = "https://fantasy.premierleague.com/api".to_string();
        Ok(FplApiClient { client, base_url })
    }

    async fn get<T: DeserializeOwned>(
        &self,
        endpoint: impl Into<String>,
        params: Option<HashMap<String, String>>,
    ) -> Result<T> {
        let url = format!("{}/{}", self.base_url, endpoint.into());

        let mut request = self.client.get(url);

        if let Some(params) = params {
            request = request.query(&params);
        }

        let response: Response = request.send().await?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP Error: {}", response.status()));
        }

        Ok(response.json::<T>().await?)
    }
}

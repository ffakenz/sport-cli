use std::time::Duration;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;

pub fn construct_url(base_url: &str, params: &[(&str, &str)]) -> String {
    let query_params: String = params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&");

    format!("{}?{}", base_url, query_params)
}

pub async fn get_json_response<T: DeserializeOwned>(client: &Client, url: &str) -> Result<T> {
    client
        .get(url)
        .header("accept", "application/json")
        .timeout(Duration::from_secs(60))
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send request: {}", e))?
        .json::<T>()
        .await
        .map_err(|e| anyhow!("Failed to parse JSON response: {}", e))
}

use std::time::Duration;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::RetryIf;

pub fn construct_url(base_url: &str, params: &[(&str, &str)]) -> String {
    let query_params: String = params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&");

    format!("{}?{}", base_url, query_params)
}

pub async fn get_json_response<T: DeserializeOwned>(client: &Client, url: &str) -> Result<T> {
    let retry_backoff_strategy =
        ExponentialBackoff::from_millis(100).max_delay(jitter(Duration::from_secs(60)));

    // TODO! add max retries
    let retry_condition = |e: &anyhow::Error| {
        // Retry if within retry limit
        print!("Retrying... Request failed: {}", e);
        true
    };

    let retry_action = || {
        let client = client.clone();
        let url = url.to_string();
        async move {
            client
                .get(&url)
                .header("accept", "application/json")
                .timeout(Duration::from_secs(60))
                .send()
                .await
                .map_err(|e| {
                    anyhow!(
                        "[{}] Failed to send request: {}",
                        e.status()
                            .map(|s| s.to_string())
                            .unwrap_or("999".to_string()),
                        e
                    )
                })?
                .json::<T>()
                .await
                .map_err(|e| {
                    anyhow!(
                        "[{}] Failed to parse JSON response: {}",
                        e.status()
                            .map(|s| s.to_string())
                            .unwrap_or("999".to_string()),
                        e
                    )
                })
        }
    };

    RetryIf::spawn(retry_backoff_strategy, retry_action, retry_condition).await
}

use anyhow::{Context, Result};
use reqwest::Client;

pub fn construct_url(base_url: &str, params: &[(&str, &str)]) -> String {
    let query_params: String = params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&");

    format!("{}?{}", base_url, query_params)
}

pub async fn get_response_text(client: &Client, url: &str) -> Result<String> {
    let response = client
        .get(url)
        .header("accept", "application/json")
        .send()
        .await
        .context("Failed to send request")?;

    let text = response
        .text()
        .await
        .context("Failed to read response text")?;

    Ok(text)
}

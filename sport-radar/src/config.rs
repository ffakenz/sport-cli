use std::env;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SportRadarConfig {
    api_key: String,
    access_level: String,
    language_code: String,
    format: String,
}

impl SportRadarConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        let api_key = env::var("API_KEY").expect("API_KEY environment variable is not set");
        let access_level = env::var("ACCESS_LEVEL").unwrap_or_else(|_| "trial".to_string());
        let language_code = env::var("LANGUAGE_CODE").unwrap_or_else(|_| "en".to_string());
        let format = env::var("FORMAT").unwrap_or_else(|_| "json".to_string());

        SportRadarConfig {
            api_key,
            access_level,
            language_code,
            format,
        }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn access_level(&self) -> &str {
        &self.access_level
    }

    pub fn language_code(&self) -> &str {
        &self.language_code
    }

    pub fn format(&self) -> &str {
        &self.format
    }
}

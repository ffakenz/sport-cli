use crate::config::SportRadarConfig;
use crate::model::{
    CompetitionsResponse, CompetitorsResponse, PlayerStatisticsResponse, SeasonsResponse,
};
use crate::utils::{construct_url, get_json_response};
use anyhow::Result;
use reqwest::Client;

pub struct SportRadarClient {
    client: Client,
    config: SportRadarConfig,
}

impl SportRadarClient {
    pub fn new(config: SportRadarConfig) -> Self {
        SportRadarClient {
            client: Client::new(),
            config,
        }
    }

    pub fn from_env() -> Self {
        let config = SportRadarConfig::from_env();
        SportRadarClient::new(config)
    }

    pub async fn get_competitions(&self) -> Result<CompetitionsResponse> {
        let endpoint = format!("{}.{}", "competitions", self.config.format());
        let base_url = format!(
            "https://api.sportradar.com/soccer/{}/v4/{}/{}",
            self.config.access_level(),
            self.config.language_code(),
            endpoint
        );
        let params = [("api_key", self.config.api_key())];
        let url = construct_url(&base_url, &params);
        dbg!("get_competitions: {:?}", &url);
        get_json_response(&self.client, &url).await
    }

    pub async fn get_competition_seasons(&self, competition_id: &str) -> Result<SeasonsResponse> {
        let endpoint = format!(
            "competitions/{}/seasons.{}",
            competition_id,
            self.config.format()
        );
        let base_url = format!(
            "https://api.sportradar.com/soccer/{}/v4/{}/{}",
            self.config.access_level(),
            self.config.language_code(),
            endpoint
        );
        let params = [("api_key", self.config.api_key())];
        let url = construct_url(&base_url, &params);
        dbg!("get_competition_seasons: {:?}", &url);
        get_json_response(&self.client, &url).await
    }

    pub async fn get_season_competitors(&self, season_id: &str) -> Result<CompetitorsResponse> {
        let endpoint = format!("seasons/{}/competitors.{}", season_id, self.config.format());
        let base_url = format!(
            "https://api.sportradar.com/soccer/{}/v4/{}/{}",
            self.config.access_level(),
            self.config.language_code(),
            endpoint
        );
        let params = [("api_key", self.config.api_key())];
        let url: String = construct_url(&base_url, &params);
        dbg!("get_season_competitors: {:?}", &url);
        get_json_response(&self.client, &url).await
    }

    pub async fn get_seasonal_competitor_statistics(
        &self,
        season_id: &str,
        competitor_id: &str,
    ) -> Result<PlayerStatisticsResponse> {
        let endpoint = format!(
            "seasons/{}/competitors/{}/statistics.{}",
            season_id,
            competitor_id,
            self.config.format()
        );
        let base_url = format!(
            "https://api.sportradar.com/soccer/{}/v4/{}/{}",
            self.config.access_level(),
            self.config.language_code(),
            endpoint
        );
        let params = [("api_key", self.config.api_key())];
        let url = construct_url(&base_url, &params);
        dbg!("get_seasonal_competitor_statistics: {:?}", &url);
        get_json_response(&self.client, &url).await
    }
}

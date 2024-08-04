use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use engine::repo::{
    Competition as EngineCompetition, Gender, Metric as RepoMetric, Player, PlayerStats,
    PlayerStatsRepo, Team,
};
use serde_derive::{Deserialize, Serialize};
use sport_radar::{
    client::SportRadarClient,
    model::{
        CompetitionGender, CompetitionsResponse, CompetitorsResponse, PlayerStatisticsResponse,
        Season, SeasonsResponse,
    },
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub event: String,
    pub location: String,
    pub gender: Gender,
    pub season_start: NaiveDate,
    pub season_end: NaiveDate,
}
unsafe impl Send for Query {}
unsafe impl Sync for Query {}

#[derive(Debug, Clone)]
pub struct Scrapper;

impl Scrapper {
    pub async fn execute(
        &self,
        sport_data_source: Arc<SportRadarClient>,
        query: Query,
    ) -> Result<PlayerStatsRepo> {
        // Initialize the repo
        let repo: Arc<Mutex<PlayerStatsRepo>> = Arc::new(Mutex::new(PlayerStatsRepo::new()));

        // Step 1: Get competitions
        println!("Step 1: Fetching competitions...");
        let competitions_response = self.get_competitions(&sport_data_source).await?;

        // Step 2: Find the competition
        println!("Step 2: Finding the competition...");
        let competition = self.find_competition(&competitions_response, &query)?;

        // Step 3: Get competition seasons
        println!("Step 3: Fetching competition seasons...");
        let seasons_response = self
            .get_seasons(&sport_data_source, &competition.id)
            .await?;

        // Step 4: Find the season
        println!("Step 4: Finding the season...");
        let season = self.find_season(&seasons_response, &query)?;

        // Step 5: Get season competitors
        println!("Step 5: Fetching season competitors...");
        let competitors_response = self.get_competitors(&sport_data_source, &season.id).await?;

        // Step 6: Fetch and process competitor statistics
        println!("Step 6: Fetching and processing competitor statistics...");
        self.fetch_and_process_stats(
            sport_data_source,
            &season.id,
            competitors_response,
            competition,
            repo.clone(),
        )
        .await?;

        // Return the repo
        println!("Completed: Returning the repository.");
        let repo = Arc::try_unwrap(repo)
            .map_err(|_| anyhow!("Failed to unwrap Arc"))?
            .into_inner()
            .map_err(|_| anyhow!("Failed to unlock Mutex"))?;
        Ok(repo)
    }

    async fn get_competitions(
        &self,
        client: &Arc<SportRadarClient>,
    ) -> Result<CompetitionsResponse> {
        client
            .get_competitions()
            .await
            .map_err(|e| anyhow!("Failed to fetch competitions: {}", e))
    }

    fn find_competition(
        &self,
        competitions_response: &CompetitionsResponse,
        query: &Query,
    ) -> Result<EngineCompetition> {
        let comp = competitions_response
            .competitions
            .iter()
            .find(|comp| {
                comp.name == query.event
                    && comp.category.name == query.location
                    && match &comp.gender {
                        Some(gender) => {
                            *gender
                                == match query.gender {
                                    Gender::Male => CompetitionGender::Men,
                                    Gender::Female => CompetitionGender::Women,
                                }
                        }
                        None => true,
                    }
            })
            .ok_or_else(|| anyhow!("Competition not found"))?;

        Ok(EngineCompetition {
            id: comp.id.clone(),
            name: comp.name.clone(),
            location: comp.category.name.clone(),
            gender: match comp.gender {
                Some(CompetitionGender::Men) => engine::repo::Gender::Male,
                Some(CompetitionGender::Women) => engine::repo::Gender::Female,
                None => return Err(anyhow!("Invalid gender in competition")),
            },
            season_start: query.season_start,
            season_end: query.season_end,
        })
    }

    async fn get_seasons(
        &self,
        client: &Arc<SportRadarClient>,
        competition_id: &str,
    ) -> Result<SeasonsResponse> {
        client
            .get_competition_seasons(competition_id)
            .await
            .map_err(|e| anyhow!("Failed to fetch competition seasons: {}", e))
    }

    fn find_season(&self, seasons_response: &SeasonsResponse, query: &Query) -> Result<Season> {
        seasons_response
            .seasons
            .iter()
            .find(|season| {
                NaiveDate::parse_from_str(&season.start_date, "%Y-%m-%d").unwrap()
                    == query.season_start
                    && NaiveDate::parse_from_str(&season.end_date, "%Y-%m-%d").unwrap()
                        == query.season_end
            })
            .cloned()
            .ok_or_else(|| anyhow!("Season not found"))
    }

    async fn get_competitors(
        &self,
        client: &Arc<SportRadarClient>,
        season_id: &str,
    ) -> Result<CompetitorsResponse> {
        client
            .get_season_competitors(season_id)
            .await
            .map_err(|e| anyhow!("Failed to fetch season competitors: {}", e))
    }

    async fn fetch_and_process_stats(
        &self,
        client: Arc<SportRadarClient>,
        season_id: &str,
        competitors_response: CompetitorsResponse,
        competition: EngineCompetition,
        repo: Arc<Mutex<PlayerStatsRepo>>,
    ) -> Result<()> {
        // TODO! optimize
        for competitor in competitors_response.season_competitors {
            let message = producer_callback(
                season_id.to_string().clone(),
                competitor.id.clone(),
                client.clone(),
            )
            .await;
            consumer_callback(message, repo.clone(), competition.clone())
        }
        Ok(())
    }
}

async fn producer_callback(
    season_id: String,
    competitor_id: String,
    client: Arc<SportRadarClient>,
) -> Result<PlayerStatisticsResponse> {
    client
        .get_seasonal_competitor_statistics(&season_id, &competitor_id)
        .await
        .map_err(|e| anyhow!("Failed to fetch competitor statistics: {}", e))
}

fn consumer_callback(
    message: Result<PlayerStatisticsResponse>,
    repo: Arc<Mutex<PlayerStatsRepo>>,
    competition: EngineCompetition,
) {
    match message {
        Err(e) => println!("Consumer received error msg: {}", e),
        Ok(_) => {
            let stats_response = message.unwrap();
            let team = Team {
                id: stats_response.competitor.id,
                name: stats_response.competitor.name,
                abbreviation: stats_response.competitor.abbreviation,
            };
            for player_stat in stats_response.competitor.players {
                let player = Player {
                    id: player_stat.id,
                    name: player_stat.name,
                    team: team.clone(),
                };

                let metrics = vec![
                    RepoMetric::GoalsScored {
                        value: player_stat.statistics.goals_scored,
                    },
                    RepoMetric::Assists {
                        value: player_stat.statistics.assists,
                    },
                ];

                let player_stats = PlayerStats {
                    player,
                    metrics,
                    competition: competition.clone(),
                };

                let mut repo = repo.lock().unwrap();
                repo.insert(player_stats);
            }
        }
    }
}

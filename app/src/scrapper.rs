use super::db::Db;
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use engine::repo::{
    in_memo::InMemoRepository,
    model::{
        Competition as EngineCompetition, Gender, Metric as RepoMetric, Player, PlayerStats, Team,
    },
};
use serde_derive::{Deserialize, Serialize};
use sport_radar::{
    client::SportRadarClient,
    model::{
        CompetitionGender, CompetitionsResponse, Competitor, CompetitorsResponse,
        PlayerStatisticsResponse, Season, SeasonsResponse,
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

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
        query: &Query,
        db: Arc<Mutex<Db>>,
    ) -> Result<()> {
        // Step 1: Get competitions
        println!("Step 1: Fetching competitions...");
        let competitions_response = self.get_competitions(&sport_data_source).await?;

        // Step 2: Find the competition
        println!("Step 2: Finding the competition...");
        let competition = self.find_competition(&competitions_response, query)?;

        // Step 3: Get competition seasons
        println!("Step 3: Fetching competition seasons...");
        let seasons_response = self
            .get_seasons(&sport_data_source, &competition.id)
            .await?;

        // Step 4: Find the season
        println!("Step 4: Finding the season...");
        let season = self.find_season(&seasons_response, query)?;

        // Step 5: Get season competitors
        println!("Step 5: Fetching season competitors...");
        let competitors_response = self.get_competitors(&sport_data_source, &season.id).await?;

        // Step 6: Fetch and process competitor statistics
        println!("Step 6: Fetching and processing competitor statistics...");
        self.process_competitor_stats(
            sport_data_source,
            &season.id,
            competitors_response,
            competition,
            db,
        )
        .await?;

        Ok(())
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
            id: Arc::new(comp.id.clone()),
            name: comp.name.clone(),
            location: comp.category.name.clone(),
            gender: match comp.gender {
                Some(CompetitionGender::Men) => Gender::Male,
                Some(CompetitionGender::Women) => Gender::Female,
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

    async fn process_competitor_stats(
        &self,
        client: Arc<SportRadarClient>,
        season_id: &str,
        competitors_response: CompetitorsResponse,
        competition: EngineCompetition,
        db: Arc<Mutex<Db>>,
    ) -> Result<()> {
        // Insert known competition
        {
            let mut db_lock = db.lock().await;
            db_lock.competitions.push(competition.clone());
        }

        // TODO! optimize
        let num_producers = 3;
        let competitors_chunks = competitors_response
            .season_competitors
            .chunks(competitors_response.season_competitors.len() / num_producers)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();

        // let mut handles = vec![];
        for chunks in competitors_chunks {
            // let handle = tokio::task::spawn(async move {
            for competitor in chunks {
                process_competitor(
                    season_id,
                    &competitor,
                    Arc::clone(&competition.id),
                    Arc::clone(&client),
                    Arc::clone(&db),
                )
                .await;
            }
            // });

            // handles.push(handle);
        }

        // for handle in handles {
        //     let _ = tokio::try_join!(handle)?;
        // }

        Ok(())
    }
}

async fn process_competitor(
    season_id: &str,
    competitor: &Competitor,
    competition_id: Arc<String>,
    client: Arc<SportRadarClient>,
    db: Arc<Mutex<Db>>,
) {
    let message: Result<PlayerStatisticsResponse> =
        producer_callback(season_id, &competitor.id, client.clone()).await;
    consumer_callback(message, competition_id, db).await;
}

async fn producer_callback(
    season_id: &str,
    competitor_id: &str,
    client: Arc<SportRadarClient>,
) -> Result<PlayerStatisticsResponse> {
    client
        .get_seasonal_competitor_statistics(season_id, competitor_id)
        .await
        .map_err(|e| anyhow!("Failed to fetch competitor statistics: {}", e))
}

async fn consumer_callback(
    message: Result<PlayerStatisticsResponse>,
    competition_id: Arc<String>,
    db: Arc<Mutex<Db>>,
) {
    match message {
        Err(e) => println!("Consumer received error msg: {}", e),
        Ok(stats_response) => {
            let team_id = Arc::new(stats_response.competitor.id);
            let team = Team {
                id: Arc::clone(&team_id),
                name: stats_response.competitor.name,
                abbreviation: stats_response.competitor.abbreviation,
            };

            let mut players = vec![];
            let mut players_stats = vec![];

            for player_stat in stats_response.competitor.players {
                let player_id = Arc::new(player_stat.id);
                let player = Player {
                    id: Arc::clone(&player_id),
                    name: player_stat.name,
                };

                players.push(player);

                let metrics = vec![
                    RepoMetric::GoalsScored {
                        value: player_stat.statistics.goals_scored,
                    },
                    RepoMetric::Assists {
                        value: player_stat.statistics.assists,
                    },
                ];

                let player_stats = PlayerStats {
                    player_id: Arc::clone(&player_id),
                    team_id: Arc::clone(&team_id),
                    competition_id: Arc::clone(&competition_id),
                    metrics,
                };

                players_stats.push(player_stats);
            }

            // Aquire db lock
            let mut db_lock = db.lock().await;
            db_lock.teams.push(team);
            for player in players {
                db_lock.players.push(player);
            }
            for player_stats in players_stats {
                db_lock.players_stats.push(player_stats);
            }
        }
    }
}

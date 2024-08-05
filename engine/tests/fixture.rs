use std::sync::Arc;

use chrono::NaiveDate;
use engine::repo::{
    competitions::CompetitionsRepo,
    in_memo::InMemoRepository,
    model::{Competition, Gender, Metric, Player, PlayerStats, Team},
    player_stats::PlayerStatsRepo,
    players::PlayersRepo,
    teams::TeamsRepo,
};

#[derive(Debug, Clone, Default)]
pub struct Fixture {
    pub competitions_repo: CompetitionsRepo,
    pub teams_repo: TeamsRepo,
    pub players_repo: PlayersRepo,
    pub player_stats_repo: PlayerStatsRepo,
}

impl Fixture {
    pub fn stub() -> Fixture {
        let team_1_id = Arc::new("sr:competitor:17".to_string());
        let team_1 = Team {
            id: Arc::clone(&team_1_id),
            name: "Manchester City".to_string(),
            abbreviation: "MCI".to_string(),
        };

        let team_2_id = Arc::new("sr:competitor:33".to_string());
        let team_2 = Team {
            id: Arc::clone(&team_2_id),
            name: "Tottenham Hotspur".to_string(),
            abbreviation: "TOT".to_string(),
        };

        let mut teams_repo: TeamsRepo = InMemoRepository::new();
        teams_repo.push(team_1);
        teams_repo.push(team_2);

        let competition_1_id = Arc::new("sr:competition:808".to_string());
        let competition_1 = Competition {
            id: Arc::clone(&competition_1_id),
            name: "Premier League".to_string(),
            location: "England".to_string(),
            gender: Gender::Male,
            season_start: NaiveDate::from_ymd_opt(2023, 8, 11).unwrap(),
            season_end: NaiveDate::from_ymd_opt(2024, 5, 19).unwrap(),
        };

        let competition_2_id = Arc::new("sr:competition:16".to_string());
        let competition_2 = Competition {
            id: Arc::clone(&competition_2_id),
            name: "World Cup".to_string(),
            location: "International".to_string(),
            gender: Gender::Male,
            season_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            season_end: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };

        let mut competitions_repo: CompetitionsRepo = InMemoRepository::new();
        competitions_repo.push(competition_1);
        competitions_repo.push(competition_2);

        let player_1_id = Arc::new("sr:player:1630398".to_string());
        let player_1 = Player {
            id: Arc::clone(&player_1_id),
            name: "Alvarez, Julian".to_string(),
        };

        let player_2_id = Arc::new("sr:player:1047129".to_string());
        let player_2 = Player {
            id: Arc::clone(&player_2_id),
            name: "Foden, Phil".to_string(),
        };

        let player_3_id = Arc::new("sr:player:952278".to_string());
        let player_3 = Player {
            id: Arc::clone(&player_3_id),
            name: "Romero, Cristian".to_string(),
        };

        let mut players_repo: PlayersRepo = InMemoRepository::new();
        players_repo.push(player_1);
        players_repo.push(player_2);
        players_repo.push(player_3);

        let player_stats_1_1 = PlayerStats {
            player_id: Arc::clone(&player_1_id),
            team_id: Arc::clone(&team_1_id),
            competition_id: Arc::clone(&competition_1_id),
            metrics: vec![
                Metric::GoalsScored { value: 10 },
                Metric::Assists { value: 3 },
            ],
        };

        let player_stats_2_1 = PlayerStats {
            player_id: Arc::clone(&player_2_id),
            team_id: Arc::clone(&team_1_id),
            competition_id: Arc::clone(&competition_1_id),
            metrics: vec![
                Metric::GoalsScored { value: 7 },
                Metric::Assists { value: 5 },
            ],
        };

        let player_stats_3_1 = PlayerStats {
            player_id: Arc::clone(&player_3_id),
            team_id: Arc::clone(&team_2_id),
            competition_id: Arc::clone(&competition_1_id),
            metrics: vec![
                Metric::GoalsScored { value: 4 },
                Metric::Assists { value: 9 },
            ],
        };

        let player_stats_1_2 = PlayerStats {
            player_id: Arc::clone(&player_1_id),
            team_id: Arc::clone(&team_1_id),
            competition_id: Arc::clone(&competition_2_id),
            metrics: vec![
                Metric::GoalsScored { value: 8 },
                Metric::Assists { value: 3 },
            ],
        };

        let player_stats_2_2 = PlayerStats {
            player_id: Arc::clone(&player_2_id),
            team_id: Arc::clone(&team_1_id),
            competition_id: Arc::clone(&competition_2_id),
            metrics: vec![
                Metric::GoalsScored { value: 9 },
                Metric::Assists { value: 4 },
            ],
        };

        let player_stats_3_2 = PlayerStats {
            player_id: Arc::clone(&player_3_id),
            team_id: Arc::clone(&team_2_id),
            competition_id: Arc::clone(&competition_2_id),
            metrics: vec![
                Metric::GoalsScored { value: 2 },
                Metric::Assists { value: 8 },
            ],
        };

        let mut player_stats_repo: PlayerStatsRepo = InMemoRepository::new();
        player_stats_repo.push(player_stats_1_1);
        player_stats_repo.push(player_stats_2_1);
        player_stats_repo.push(player_stats_3_1);
        player_stats_repo.push(player_stats_1_2);
        player_stats_repo.push(player_stats_2_2);
        player_stats_repo.push(player_stats_3_2);

        Fixture {
            competitions_repo,
            teams_repo,
            players_repo,
            player_stats_repo,
        }
    }
}

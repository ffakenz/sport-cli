use chrono::NaiveDate;
use engine::repo::{Competition, Gender, Metric, Player, PlayerStats, PlayerStatsRepo, Team};

pub fn repo_stub() -> PlayerStatsRepo {
    let team_1 = Team {
        id: "sr:competitor:17".to_string(),
        name: "Manchester City".to_string(),
        abbreviation: "MCI".to_string(),
    };

    let team_2 = Team {
        id: "sr:competitor:33".to_string(),
        name: "Tottenham Hotspur".to_string(),
        abbreviation: "TOT".to_string(),
    };

    let competition_1 = Competition {
        id: "sr:competition:808".to_string(),
        name: "Premier League".to_string(),
        location: "England".to_string(),
        gender: Gender::Male,
        season_start: NaiveDate::from_ymd_opt(2023, 8, 11).unwrap(),
        season_end: NaiveDate::from_ymd_opt(2024, 5, 19).unwrap(),
    };

    let competition_2 = Competition {
        id: "sr:competition:16".to_string(),
        name: "World Cup".to_string(),
        location: "International".to_string(),
        gender: Gender::Male,
        season_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        season_end: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let player_1 = Player {
        id: "sr:player:1630398".to_string(),
        name: "Alvarez, Julian".to_string(),
        team: team_1.clone(),
    };

    let player_2 = Player {
        id: "sr:player:1047129".to_string(),
        name: "Foden, Phil".to_string(),
        team: team_1.clone(),
    };

    let player_3 = Player {
        id: "sr:player:952278".to_string(),
        name: "Romero, Cristian".to_string(),
        team: team_2.clone(),
    };

    let player_stats_1_1 = PlayerStats {
        player: player_1.clone(),
        metrics: vec![
            Metric::GoalsScored { value: 10 },
            Metric::Assists { value: 3 },
        ],
        competition: competition_1.clone(),
    };

    let player_stats_2_1 = PlayerStats {
        player: player_2.clone(),
        metrics: vec![
            Metric::GoalsScored { value: 7 },
            Metric::Assists { value: 5 },
        ],
        competition: competition_1.clone(),
    };

    let player_stats_3_1 = PlayerStats {
        player: player_3.clone(),
        metrics: vec![
            Metric::GoalsScored { value: 4 },
            Metric::Assists { value: 9 },
        ],
        competition: competition_1.clone(),
    };

    let player_stats_1_2 = PlayerStats {
        player: player_1.clone(),
        metrics: vec![
            Metric::GoalsScored { value: 8 },
            Metric::Assists { value: 3 },
        ],
        competition: competition_2.clone(),
    };

    let player_stats_2_2 = PlayerStats {
        player: player_2.clone(),
        metrics: vec![
            Metric::GoalsScored { value: 9 },
            Metric::Assists { value: 4 },
        ],
        competition: competition_2.clone(),
    };

    let player_stats_3_2 = PlayerStats {
        player: player_3.clone(),
        metrics: vec![
            Metric::GoalsScored { value: 2 },
            Metric::Assists { value: 8 },
        ],
        competition: competition_2.clone(),
    };

    let mut repo = PlayerStatsRepo::new();
    repo.insert(player_stats_1_1);
    repo.insert(player_stats_2_1);
    repo.insert(player_stats_3_1);
    repo.insert(player_stats_1_2);
    repo.insert(player_stats_2_2);
    repo.insert(player_stats_3_2);
    repo
}

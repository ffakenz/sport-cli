use anyhow::Result;
use chrono::NaiveDate;
use engine::{
    engine::*,
    repo::{Gender, Player},
};

mod fixture;

#[test]
fn top_2_score_players() -> Result<()> {
    let repo = fixture::repo_stub();
    let engine = Engine;
    let query = Query {
        event: "Premier League".to_string(),
        location: "England".to_string(),
        season_start: NaiveDate::from_ymd_opt(2023, 8, 11).unwrap(),
        season_end: NaiveDate::from_ymd_opt(2024, 5, 19).unwrap(),
        gender: Gender::Male,
        dimension: Dimension::Player,
        metric: Metric::GoalsScored,
        sort: Sort::Desc,
        limit: 2,
    };
    let result: Vec<Player> = engine.execute(repo, query);
    let result_ids: Vec<String> = result.into_iter().map(|p| p.id).collect();

    let expected_ids = vec![
        "sr:player:1630398".to_string(),
        "sr:player:1047129".to_string(),
    ];

    assert_eq!(
        result_ids, expected_ids,
        "The result did not match the expected output"
    );

    Ok(())
}

#[test]
fn top_2_assist_players() -> Result<()> {
    let repo = fixture::repo_stub();
    let engine = Engine;
    let query = Query {
        event: "Premier League".to_string(),
        location: "England".to_string(),
        season_start: NaiveDate::from_ymd_opt(2023, 8, 11).unwrap(),
        season_end: NaiveDate::from_ymd_opt(2024, 5, 19).unwrap(),
        gender: Gender::Male,
        dimension: Dimension::Player,
        metric: Metric::Assists,
        sort: Sort::Desc,
        limit: 2,
    };
    let result: Vec<Player> = engine.execute(repo, query);
    let result_ids: Vec<String> = result.into_iter().map(|p| p.id).collect();

    let expected_ids = vec![
        "sr:player:952278".to_string(),
        "sr:player:1047129".to_string(),
    ];

    assert_eq!(
        result_ids, expected_ids,
        "The result did not match the expected output"
    );

    Ok(())
}

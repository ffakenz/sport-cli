use anyhow::Result;
use serde_json::from_str;
use sport_radar::model::{
    CompetitionsResponse, CompetitorsResponse, PlayerStatisticsResponse, SeasonsResponse,
};
use std::{env, fs, path::PathBuf};

fn load_resource(file_name: &str) -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());
    let file = format!("resources/api/{}", file_name);
    let path: PathBuf = [current_dir.to_str().unwrap(), &file].iter().collect();
    println!("Checking file path: {}", path.display());
    assert!(path.exists(), "File does not exist: {}", path.display());
    Ok(path)
}

#[test]
fn test_parse_competitions() -> Result<()> {
    let path = load_resource("competitions.json")?;
    let file_content = fs::read_to_string(path)?;
    let _: CompetitionsResponse = from_str(&file_content)?;
    // println!("{:?}", competition);
    Ok(())
}

#[test]
fn test_parse_seasons() -> Result<()> {
    let path = load_resource("competition_seasons.json")?;
    let file_content = fs::read_to_string(path)?;
    let _: SeasonsResponse = from_str(&file_content)?;
    // println!("{:?}", season);
    Ok(())
}

#[test]
fn test_parse_competitor() -> Result<()> {
    let path = load_resource("season_competitors.json")?;
    let file_content = fs::read_to_string(path)?;
    let _: CompetitorsResponse = from_str(&file_content)?;
    // println!("{:?}", competitor);
    Ok(())
}

#[test]
fn test_parse_player_statistics() -> Result<()> {
    let path = load_resource("seasonal_competitor_statistics.json")?;
    let file_content = fs::read_to_string(path)?;
    let _: PlayerStatisticsResponse = from_str(&file_content)?;
    // println!("{:?}", player_statistics);
    Ok(())
}

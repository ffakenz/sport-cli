use anyhow::Result;
use sport_radar::client::SportRadarClient;

#[tokio_macros::test]
async fn test_get_competitions() -> Result<()> {
    let client = SportRadarClient::from_env();

    let competitions = client.get_competitions().await;

    assert!(competitions.is_ok(), "Competitions api call failed");

    Ok(())
}

#[tokio_macros::test]
async fn test_get_competition_seasons() -> Result<()> {
    let client = SportRadarClient::from_env();

    let seasons = client.get_competition_seasons("sr:competition:17").await;

    assert!(seasons.is_ok(), "Seasons api call failed");

    Ok(())
}

#[tokio_macros::test]
async fn test_get_season_competitors() -> Result<()> {
    let client = SportRadarClient::from_env();

    let competitors = client.get_season_competitors("sr:season:105353").await;

    assert!(competitors.is_ok(), "Competitors api call failed");

    Ok(())
}

#[tokio_macros::test]
async fn test_get_seasonal_competitor_statistics() -> Result<()> {
    let client = SportRadarClient::from_env();

    let statistics = client
        .get_seasonal_competitor_statistics("sr:season:105353", "sr:competitor:44")
        .await;

    assert!(statistics.is_ok(), "Player statistics api call failed");

    Ok(())
}

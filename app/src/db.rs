use engine::repo::{
    competitions::CompetitionsRepo, player_stats::PlayerStatsRepo, players::PlayersRepo,
    teams::TeamsRepo,
};

#[derive(Debug, Clone, Default)]
pub struct Db {
    pub players: PlayersRepo,
    pub competitions: CompetitionsRepo,
    pub teams: TeamsRepo,
    pub players_stats: PlayerStatsRepo,
}

impl Db {
    pub fn new() -> Self {
        Self::default()
    }
}

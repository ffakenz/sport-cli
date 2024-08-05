mod db;

mod cli {
    pub mod analytics {
        pub mod args;
        pub mod run;
    }
    pub mod competitions {
        pub mod args;
    }
    pub mod players {
        pub mod args;
    }
    pub mod seasons {
        pub mod args;
    }
    pub mod teams {
        pub mod args;
    }
    pub mod args;
    pub mod utils;
}

use cli::args::*;

mod scrapper;
use scrapper::Query;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analytics(args) => {
            println!("Analytics Query: {:?}", args);
            let query = Query {
                event: args.event.to_string(),
                location: args.location.to_string(),
                gender: args.gender.value.clone(),
                season_start: args.season_start,
                season_end: args.season_end,
            };
            cli::analytics::run::run(query).await
        }
        _ => todo!(),
    }
}

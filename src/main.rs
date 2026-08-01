mod search;
use clap::{ArgAction, Parser};
use crate::search::run_search;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(version = "v0.0.1", disable_help_flag = true, disable_version_flag = true)]
struct Args {
    #[arg(long, help = "Port", default_value_t = 2999)]
    port: u16,

    #[arg(long, action = ArgAction::Help, help = "Print help")]
    help: Option<bool>,

    #[arg(long, action = ArgAction::Version, help = "Print version")]
    version: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("{args:?}");

    let _ = sea_orm::Database::connect("sqlite://app.db?mode=rwc").await?;
    println!("Connected");

    if let Ok(selected) = run_search() {
        println!("Selected: {:?}", selected);
    }
    Ok(())
}

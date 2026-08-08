mod app;
mod cli;
mod db;
mod paths;
mod search;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    app::run(cli::Args::parse()).await
}

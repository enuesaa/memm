mod search;
mod migrator;
mod entities;

use clap::{Parser, Subcommand};
use crate::search::run_search;
use crate::migrator::Migrator;
use crate::entities::memos;
use sea_orm_migration::MigratorTrait;
use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use chrono::Utc;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(version = "v0.0.1")]
struct Args {
    #[arg(long, help = "Port", default_value_t = 2999)]
    port: u16,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Search memos (default)
    Search,
    /// Add a new memo
    Add {
        #[arg(long)]
        title: String,

        #[arg(long, default_value = "")]
        description: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("{args:?}");

    let db = sea_orm::Database::connect("sqlite://app.db?mode=rwc").await?;
    println!("Connected");

    Migrator::up(&db, None).await?;
    println!("Migrated");

    match args.command.unwrap_or(Command::Search) {
        Command::Search => {
            let memo_list = memos::Entity::find().all(&db).await?;
            let items: Vec<(i32, String)> = memo_list
                .into_iter()
                .map(|m| (m.id, m.title))
                .collect();

            if let Ok(selected) = run_search(items) {
                println!("Selected: {:?}", selected);
            }
        }
        Command::Add { title, description } => {
            let now = Utc::now();

            let memo = memos::ActiveModel {
                title: Set(title),
                description: Set(description),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };

            let inserted = memo.insert(&db).await?;
            println!("Inserted: {:?}", inserted);
        }
    }

    Ok(())
}

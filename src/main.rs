mod search;
mod migrator;
mod entities;
mod cli;

use clap::Parser;
use crate::cli::cmd;
use crate::migrator::Migrator;
use crate::search::run::run_search;
use crate::entities::memos;
use sea_orm_migration::MigratorTrait;
use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use chrono::Utc;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cmd::Args::parse();
    println!("{args:?}");

    let db = sea_orm::Database::connect("sqlite://app.db?mode=rwc").await?;
    println!("Connected");

    Migrator::up(&db, None).await?;
    println!("Migrated");

    match args.command.unwrap_or(cmd::Command::Search) {
        cmd::Command::Search => {
            let memo_list = memos::Entity::find().all(&db).await?;
            let items: Vec<(i32, String)> = memo_list
                .into_iter()
                .map(|m| (m.id, m.title))
                .collect();

            if let Ok(selected) = run_search(items) {
                println!("Selected: {:?}", selected);
            }
        }
        cmd::Command::Add { title, description } => {
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

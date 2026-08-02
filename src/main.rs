mod search;
mod migrator;
mod entities;
mod cli;
mod repositories;
mod fs;

use anyhow::Result;
use clap::Parser;
use repositories::memos::MemoRepository;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();
    println!("{args:?}");

    fs::mk_data_dir()?;

    let dburi = fs::get_db_uri()?;
    println!("dburi: {}", dburi);
    let db = sea_orm::Database::connect(dburi).await?;
    migrator::migrate(&db).await?;
    println!("Migrated");

    match args.command.unwrap_or(cli::Command::Search) {
        cli::Command::Search => {
            let memo_list = MemoRepository::find_all(&db).await?;
            let items: Vec<(i32, String)> = memo_list
                .into_iter()
                .map(|m| (m.id, m.title))
                .collect();
            if let Ok(selected) = search::search(items) {
                println!("Selected: {:?}", selected);
            }
        }

        cli::Command::Add { title, description } => {
            let inserted = MemoRepository::create(&db, title, description).await?;
            println!("Inserted: {:?}", inserted);
        }
    }
    Ok(())
}
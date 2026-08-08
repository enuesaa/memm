use anyhow::{Result, anyhow};
use std::env;
use std::fs;
use std::path::PathBuf;

/* data dir */
pub fn data_dir() -> Result<PathBuf> {
    let home = env::home_dir().ok_or_else(|| anyhow!("failed to get home dir"))?;
    Ok(home.join(".memm"))
}

pub fn mk_data_dir() -> Result<()> {
    fs::create_dir_all(data_dir()?)?;
    Ok(())
}

/* db file */
pub fn db_file() -> Result<PathBuf> {
    Ok(data_dir()?.join("app.db"))
}

pub fn db_uri() -> Result<String> {
    let path = db_file()?;
    Ok(format!("sqlite://{}?mode=rwc", path.to_string_lossy()))
}

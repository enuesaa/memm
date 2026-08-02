use anyhow::{Result, anyhow};
use std::env;
use std::fs;
use std::path::PathBuf;

/* data dir */
fn get_data_dir() -> Result<PathBuf> {
    let home = env::home_dir().ok_or_else(|| anyhow!("failed to get home dir"))?;
    let dir = home.join(".memm");
    Ok(dir)
}

fn is_data_dir_exist() -> Result<bool> {
    let dir = get_data_dir()?;
    Ok(fs::exists(dir)?)
}

pub fn mk_data_dir() -> Result<()> {
    if is_data_dir_exist()? {
        return Ok(());
    }
    let dir = get_data_dir()?;
    let _ = fs::create_dir(dir)?;
    Ok(())
}

/* db file */
fn get_db_file_path() -> Result<PathBuf> {
    let dir = get_data_dir()?;
    let path = dir.join("app.db");
    Ok(path)
}

pub fn get_db_uri() -> Result<String> {
    let path = get_db_file_path()?;
    let uri = format!("sqlite://{}?mode=rwc", path.to_string_lossy());
    Ok(uri)
}

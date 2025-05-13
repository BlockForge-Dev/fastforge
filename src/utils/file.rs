// src/utils/file.rs
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

pub fn get_cache_path() -> PathBuf {
    let mut path = dirs::cache_dir().unwrap_or_else(|| PathBuf::from(".cache"));
    path.push("fastforge");
    path
}

pub fn save_token(token: &str) -> std::io::Result<()> {
    let mut path = get_cache_path();
    create_dir_all(&path)?;
    path.push("auth_token");

    write(path, token)?;
    Ok(())
}



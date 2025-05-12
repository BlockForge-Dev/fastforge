use crate::models::auth::AuthToken;
use chrono::Utc;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::env;
use std::fs::File;


pub fn save_token(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let created_at = Utc::now().to_rfc3339();
    let data = AuthToken {
        token: token.to_string(),
        created_at,
    };

    let path = env::var("AUTH_CACHE_PATH")?;
    let json = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn read_token() -> Option<AuthToken> {
    let path = env::var("AUTH_CACHE_PATH").ok()?;
    if !Path::new(&path).exists() {
        return None;
    }

    let mut file = File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    serde_json::from_str(&contents).ok()
}

pub fn clear_token() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::var("AUTH_CACHE_PATH")?;
    if Path::new(&path).exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

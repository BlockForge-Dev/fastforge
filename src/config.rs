use std::env;
use dotenv::dotenv;
pub struct Config {
    pub api_key: String,
    pub app_name: String,
}

impl Config{
    pub fn from_env()->Self{
        dotenv().ok();
        Self {
            api_key:
             env::var("API_KEY").expect("API_KEY is missing"),
            app_name:
            env::var("APP_NAME").unwrap_or_else(|_| "FastForge".to_string()),
        }
    }
}
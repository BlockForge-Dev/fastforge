use dotenv::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing env var: {}", key))
}

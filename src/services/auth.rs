// src/services/auth.rs
use crate::models::auth::{LoginRequest, LoginResponse};
use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

pub async fn login_user(req: LoginRequest) -> Result<LoginResponse> {
    // Simulate delay (API call)
    sleep(Duration::from_millis(500)).await;

    // Stubbed response
    if req.email == "demo@fastforge.dev" && req.password == "password" {
        Ok(LoginResponse {
            token: "mocked_token_1234567890".to_string(),
            user_id: "user_demo".to_string(),
            email: req.email,
        })
    } else {
        anyhow::bail!("Invalid credentials")
    }
}

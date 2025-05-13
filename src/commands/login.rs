// src/commands/login.rs
use crate::services::auth::login_user;
use crate::utils::file::save_token;
use crate::models::auth::LoginRequest;

use anyhow::Result;
use dialoguer::{Input, Password};

pub async fn handle_login() -> Result<()> {
    let email: String = Input::new()
        .with_prompt("Email")
        .interact_text()?;

    let password: String = Password::new()
        .with_prompt("Password")
        .interact()?;

    let req = LoginRequest { email, password };
    let res = login_user(req).await?;

    save_token(&res.token)?;
    println!("✅ Logged in as {}", res.email);
    Ok(())
}

use crate::services::auth::save_token;

pub fn handle_login(token: String) {
    match save_token(&token) {
        Ok(_) => println!("✅ Logged in successfully."),
        Err(e) => eprintln!("❌ Failed to save token: {}", e),
    }
}

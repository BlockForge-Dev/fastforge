use crate::services::auth::clear_token;

pub fn handle_logout() {
    match clear_token() {
        Ok(_) => println!("👋 Logged out successfully."),
        Err(e) => eprintln!("❌ Failed to clear token: {}", e),
    }
}

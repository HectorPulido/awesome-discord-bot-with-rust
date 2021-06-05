use super::custom_database::DiscordDatabase;
use std::sync::Mutex;

#[allow(dead_code)]
pub struct AppState {
    pub bd: Mutex<DiscordDatabase>,
}

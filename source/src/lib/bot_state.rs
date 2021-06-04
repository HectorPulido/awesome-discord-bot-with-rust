use super::custom_database::DiscordDatabase;
use discord::model::Message;
use discord::Discord;

pub struct BotState {
    pub discord: Discord,
    pub db: DiscordDatabase,
    pub last_command: Option<Message>,
    pub last_command_output: String,
}

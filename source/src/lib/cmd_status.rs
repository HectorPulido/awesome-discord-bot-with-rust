use super::bot_state::BotState;
use discord::model::Message;

#[allow(dead_code)]
pub fn cmd_status(state: &mut BotState, _: &Message) -> String {
    if state.last_command_output.is_empty() {
        return "No hay nada que reportar".to_string();
    }

    return state.last_command_output.clone();
}

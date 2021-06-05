use super::super::bot_state::BotState;
use discord::model::Message;

#[allow(dead_code)]
pub fn cmd_test(state: &mut BotState, message: &Message) -> String {
    let author = &message.author.name;
    let response = format!("Has dicho hola {}", author);
    state.last_command_output = "".to_string();
    return response;
}

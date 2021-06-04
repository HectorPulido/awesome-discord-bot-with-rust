use super::bot_state::BotState;
use super::custom_database::Resource;
use discord::model::{Message, ReactionEmoji};

#[allow(dead_code)]
pub fn cmd_save_resource(state: &mut BotState, message: &Message) -> String {
    let resource = Resource::new(message);
    let save_result = state.db.insert_resource(resource);

    let unicode_reaction;
    if save_result {
        unicode_reaction = "👍";
        state.last_command_output = "".to_string();
    } else {
        unicode_reaction = "👎";

        state.last_command_output = format!(
            "No se pudo salvar la informacion de el mensaje de {}",
            message.author.mention()
        );
    }

    let reaction = ReactionEmoji::Unicode(unicode_reaction.to_string());

    state
        .discord
        .add_reaction(message.channel_id, message.id, reaction)
        .unwrap();

    return "".to_string();
}

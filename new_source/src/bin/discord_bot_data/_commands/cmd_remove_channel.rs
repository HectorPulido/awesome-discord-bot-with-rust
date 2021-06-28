extern crate awesome_discord_bot;

use super::super::bot_state::BotState;
use super::utils::{add_thumbs_up, is_admin};
use awesome_discord_bot::models::channels::Channels;
use discord::model::Message;

#[allow(dead_code)]
pub fn cmd_remove_channel(state: &mut BotState, message: &Message) -> Option<String> {
    // Check permissions
    let mention = message.author.mention();
    let channel_id = &message.channel_id.0;

    if !is_admin(state, message) {
        let msg: String = format!("No tienes permiso para realizar esta accion {}", mention);
        return Some(msg);
    }

    match Channels::remove_channel(&state.db, channel_id) {
        Ok(_) => {
            add_thumbs_up(state, message);
            return None;
        }
        Err(_) => {
            let msg: String = format!(
                "{} No se pudo eliminar el canal, contacta con un administrador",
                mention
            );

            return Some(msg);
        }
    }
}

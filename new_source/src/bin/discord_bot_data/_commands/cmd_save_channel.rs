extern crate awesome_discord_bot;

use super::super::bot_state::BotState;
use super::utils::is_admin;
use awesome_discord_bot::models::channels::Channels;
use awesome_discord_bot::models::types::Types;
use discord::model::Message;
use regex::Regex;

#[allow(dead_code)]
pub fn cmd_save_channel(state: &mut BotState, message: &Message) -> Option<String> {
    // Check permissions
    let mention = message.author.mention();
    if !is_admin(state, message) {
        let msg: String = format!("No tienes permiso para realizar esta accion {}", mention);
        return Some(msg);
    }

    let input = message.content.as_str();
    let channel_id = &message.channel_id.0;
    let re_query: Regex = Regex::new(r"!save_channel (?P<type>.*)").unwrap();

    let query = re_query
        .captures(input)
        .and_then(|cap| cap.name("type").map(|m| m.as_str()))
        .unwrap();

    let type_data: Option<Types> = match Types::type_exist(&state.db, query) {
        Some(t) => Some(t),
        None => {
            // Create type
            let created_type = Types::create_type(&state.db, query);
            match created_type {
                Ok(t) => Some(t),
                Err(_) => None,
            }
        }
    };

    match type_data {
        Some(t) => {
            // add channel
            match Channels::create_channel(&state.db, channel_id, &t.id) {
                Ok(_) => {
                    let msg: String = format!(
                        "Canal añadido correctamente con el tipo {}, {}",
                        t.type_description, mention
                    );
                    return Some(msg);
                }
                Err(_) => {
                    let msg: String = format!(
                        "No se pudo agregar el canal, probablemente el canal ya esta agregado {}",
                        mention
                    );
                    return Some(msg);
                }
            }
        }
        None => {
            let msg: String = format!(
                "No se pudo agregar el canal, probablemente el canal ya esta agregado {}",
                mention
            );
            return Some(msg);
        }
    };
}

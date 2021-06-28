// use super::super::bot_state::BotState;
// use discord::model::{permissions, Channel, Message, ReactionEmoji};
// use discord::ChannelRef;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
use serenity::model::permissions::Permissions;
use serenity::prelude::Context;

pub fn get_mention(message: &Message) -> String {
    return format!("<@{}>", message.author.id);
}

pub async fn is_admin(ctx: &Context, msg: &Message) -> bool {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            let r = role.to_role_cached(&ctx.cache).await;
            if r.map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR)) {
                return true;
            }
        }
    }
    return false;
}

pub async fn add_thumbs_down(context: &Context, message: &Message) {
    let unicode_reaction = String::from("👎");
    let _ = add_reaction(unicode_reaction, context, message).await;
}

pub async fn add_thumbs_up(context: &Context, message: &Message) {
    let unicode_reaction = String::from("👎");
    let _ = add_reaction(unicode_reaction, context, message).await;
}
pub async fn add_reaction(unicode_reaction: String, context: &Context, message: &Message) {
    let reaction = ReactionType::Unicode(unicode_reaction);

    let _ = context
        .http
        .clone()
        .create_reaction(message.channel_id.0, message.id.0, &reaction)
        .await;
}

// #[allow(dead_code)]
// pub fn response(message: &str) -> Option<String> {
//     return Some(String::from(message));
// }

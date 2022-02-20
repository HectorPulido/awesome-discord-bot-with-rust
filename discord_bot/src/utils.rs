use serde_json::Value;
use serenity::model::{
    channel::{Message, ReactionType},
    id::UserId,
};
use serenity::prelude::Context;

pub struct Handler {
    pub owner_id: UserId,
    pub bot_id: UserId,
    pub endpoint: String,
    pub key: String,
    pub name: String,
    pub client: reqwest::Client,
    pub channel_data: Option<Value>,
}

#[allow(dead_code)]
pub fn author_is_owner(s: &Handler, msg: &Message) -> bool {
    return msg.author.id == s.owner_id;
}

pub fn mentions_me(s: &Handler, msg: &Message) -> bool {
    return msg.mentions_user_id(s.bot_id);
}

pub async fn add_reaction(unicode_reaction: String, context: &Context, message: &Message) {
    let reaction = ReactionType::Unicode(unicode_reaction);

    let _ = context
        .http
        .clone()
        .create_reaction(message.channel_id.0, message.id.0, &reaction)
        .await;
}

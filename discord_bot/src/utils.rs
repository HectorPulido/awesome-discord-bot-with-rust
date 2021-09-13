use regex::Regex;
use serde_json::Value;
use serenity::model::{
    channel::{Message, ReactionType},
    id::UserId,
};
use serenity::prelude::Context;
use std::collections::HashMap;

pub struct Handler {
    pub owner_id: UserId,
    pub bot_id: UserId,
    pub endpoint: String,
    pub key: String,
    pub name: String,
    pub client: reqwest::Client,
    pub channel_data: Value,
}

#[allow(dead_code)]
pub fn author_is_owner(s: &Handler, msg: &Message) -> bool {
    return msg.author.id == s.owner_id;
}

pub fn mentions_me(s: &Handler, msg: &Message) -> bool {
    return msg.mentions_user_id(s.bot_id);
}

pub fn get_credentials(s: &Handler) -> HashMap<&str, String> {
    let mut map = HashMap::new();
    map.insert("name", s.name.to_string());
    map.insert("private_key", s.key.to_string());
    return map;
}

pub fn clean_text(s: &str) -> String {
    let re = Regex::new(r"<@!\d+>").unwrap();
    let result = re.replace_all(s, "");
    let result = result.to_string().trim().to_string();

    return result;
}

pub fn get_mention(message: &Message) -> String {
    return format!("<@{}>", message.author.id);
}

pub fn post_process(s: &str, message: &Message) -> Vec<String> {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    let chars = chars.as_str();

    let chars = format!("{} {}", chars, get_mention(message));

    let splitted: Vec<String> = chars.split("\\n").map(|x| x.to_string()).collect();
    return splitted;
}

pub fn get_links(s: &str) -> Vec<String> {
    println!("{:?}", s);

    let re_title = Regex::new(r#"(^|\s|\n)*(https?://[^\s]+)($|\s|\n)*"#).unwrap();

    let mut vec = Vec::new();

    for cap in re_title.captures_iter(s) {
        vec.push(String::from(&cap[2]));
    }

    return vec;
}

#[allow(dead_code)]
pub async fn add_thumbs_down(context: &Context, message: &Message) {
    let unicode_reaction = String::from("👎");
    let _ = add_reaction(unicode_reaction, context, message).await;
}

pub async fn add_thumbs_up(context: &Context, message: &Message) {
    let unicode_reaction = String::from("👍");
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

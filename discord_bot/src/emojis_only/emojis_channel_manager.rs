use crate::utils::Handler;
use regex::Regex;
use serenity::{model::channel::Message, prelude::*};

pub async fn manage_emojis_channel(_: &Handler, ctx: &Context, msg: &Message) {
    // Remove all discord emojis
    let message = msg.content.to_lowercase();

    let re = Regex::new(r"<[^\s]+>").unwrap();
    let message = re.replace_all(&message, "").to_string();

    let re = Regex::new(r"[a-z0-9@\.\-=\*\&\^%\$\#@!`\~\?]").unwrap();
    for _ in re.captures_iter(&message) {
        msg.delete(&ctx).await.unwrap();
        return;
    }
}

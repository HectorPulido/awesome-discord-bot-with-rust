use crate::utils::Handler;
use regex::Regex;
use serenity::{model::channel::Message, prelude::*};

fn contains_ilegal_characters(message: String) -> bool {
    // Remove all discord emojis
    let re = Regex::new(r"<[^\s]+>").unwrap();
    let message = re.replace_all(&message, "").to_string();

    // iIlegal characters
    let re = Regex::new(r"[a-z0-9@\.\-=\*\&\^%\$\#@!`\~\?]").unwrap();
    for _ in re.captures_iter(&message) {
        return true;
    }

    false
}

pub async fn manage_emojis_channel(_: &Handler, ctx: &Context, msg: &Message) {
    let message = msg.content.to_lowercase();

    if contains_ilegal_characters(message) {
        msg.delete(&ctx).await.unwrap();
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_ilegal_characters() {
        let message = "test".to_string();
        assert!(contains_ilegal_characters(message));
    }

    #[test]
    fn test_contains_no_ilegal_characters() {
        let message = "<test_emoji><test_emoji2>".to_string();
        assert!(!contains_ilegal_characters(message));
    }

    #[test]
    fn test_contains_no_ilegal_characters_2() {
        let message = "🔥🐻".to_string();
        assert!(!contains_ilegal_characters(message));
    }

    #[test]
    fn test_contains_no_ilegal_characters_3() {
        let message = "(╯°□°）╯︵ ┻━┻".to_string();
        assert!(!contains_ilegal_characters(message));
    }
}

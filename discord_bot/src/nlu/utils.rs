use regex::Regex;
use serenity::model::channel::Message;

pub fn clean_text(s: &str) -> String {
    let re = Regex::new(r"<@!?\d+>").unwrap();
    let result = re.replace_all(s, "");

    result.to_string().trim().to_string()
}

pub fn remove_links(s: &str) -> String {
    let re = Regex::new(r"https?://[^\s]+").unwrap();
    let result = re.replace_all(s, "");

    result.to_string().trim().to_string()
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

pub fn get_mention(message: &Message) -> String {
    return format!("<@{}>", message.author.id);
}

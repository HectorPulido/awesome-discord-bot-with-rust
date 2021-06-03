use super::custom_database::{DiscordDatabase, Resource};

use discord::model::ReactionEmoji;
use discord::{model, Discord};

use regex::Regex;

pub fn cmd_test(_: &Discord, message: &model::Message, _: &mut DiscordDatabase) -> String {
    let author = &message.author.name;
    let response = format!("Has dicho hola {}", author);
    return response;
}

pub fn cmd_save_resource(
    discord: &Discord,
    message: &model::Message,
    ddbs: &mut DiscordDatabase,
) -> String {
    let resource = Resource::new(message);
    let save_result = ddbs._insert_resource(resource);

    let unicode_reaction = if save_result { "👍" } else { "👎" };
    let reaction = ReactionEmoji::Unicode(unicode_reaction.to_string());

    discord
        .add_reaction(message.channel_id, message.id, reaction)
        .unwrap();

    return "".to_string();
}

pub fn cmd_search_resource(
    _: &Discord,
    message: &model::Message,
    ddbs: &mut DiscordDatabase,
) -> String {
    let input = message.content.as_str();
    let re_query: Regex = Regex::new(r"!search (?P<query>.*)").unwrap();

    let query = re_query
        .captures(input)
        .and_then(|cap| cap.name("query").map(|login| login.as_str()))
        .unwrap();

    let resource = ddbs._select_random_resource(query);

    let author_mention = message.author.mention();

    let mut response = format!(
        "No he podido encontrar nada relacionado {} \
        por favor prueba con otras palabras claves",
        author_mention
    );

    if resource.len() > 0 {
        response = format!(
            "He encontrado eso {} espero que te sirva {}",
            author_mention, resource[0].url
        );
    }

    return response.to_string();
}

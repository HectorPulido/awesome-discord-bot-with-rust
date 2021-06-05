use super::super::bot_state::BotState;
use discord::model::Message;
use regex::Regex;

#[allow(dead_code)]
pub fn cmd_search_resource(state: &mut BotState, message: &Message) -> String {
    let input = message.content.as_str();
    let re_query: Regex = Regex::new(r"!search (?P<query>.*)").unwrap();

    let query = re_query
        .captures(input)
        .and_then(|cap| cap.name("query").map(|login| login.as_str()))
        .unwrap();

    let resource = state.db.select_random_resource(query);

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

    state.last_command_output = "".to_string();

    return response.to_string();
}

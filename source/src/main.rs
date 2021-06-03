extern crate discord;

mod lib;

use discord::model::{Event, Message};
use discord::Discord;
use lib::cmd_test::{cmd_save_resource, cmd_search_resource, cmd_test};
use lib::custom_database::DiscordDatabase;
use regex::Regex;
use std::env;

fn commands_array() -> Vec<(
    Regex,
    fn(d: &Discord, msg: &Message, ddb: &mut DiscordDatabase) -> String,
)> {
    let mut commands: Vec<(
        Regex,
        fn(d: &Discord, msg: &Message, ddb: &mut DiscordDatabase) -> String,
    )> = Vec::new();
    commands.push((Regex::new(r"^!test").unwrap(), cmd_test));
    commands.push((Regex::new(r"^!save ").unwrap(), cmd_save_resource));
    commands.push((Regex::new(r"^!search ").unwrap(), cmd_search_resource));

    return commands;
}

fn process_message(
    discord: &Discord,
    message: &Message,
    commands_array: &Vec<(
        Regex,
        fn(d: &Discord, m: &Message, db: &mut DiscordDatabase) -> String,
    )>,
    db: &mut DiscordDatabase,
) {
    let content: &str = message.content.as_str();
    for (command, function) in commands_array {
        if command.is_match(content) {
            let response = function(discord, message, db);
            let str_response = response.as_str();
            if response.len() != 0 {
                let _ = discord.send_message(message.channel_id, str_response, "", false);
            }
            break;
        }
    }
}

fn main() {
    // Get env data
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let database_uri = env::var("DATABASE_URI").expect("Expected a token in the environment");

    // Database
    let mut db_client = DiscordDatabase::new(database_uri);

    // Test event
    let commands_array = commands_array();

    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(&token).expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");

    // Event loop
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                process_message(&discord, &message, &commands_array, &mut db_client);
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}

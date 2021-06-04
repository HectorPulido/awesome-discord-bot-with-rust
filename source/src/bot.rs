extern crate discord;

mod lib;

use discord::model::{Event, Message};
use discord::Discord;
use lib::bot_state::BotState;
use lib::cmd_save_resource::cmd_save_resource;
use lib::cmd_search_resource::cmd_search_resource;
use lib::cmd_status::cmd_status;
use lib::cmd_test::cmd_test;
use lib::custom_database::DiscordDatabase;
use regex::Regex;
use std::env;

fn commands_array() -> Vec<(Regex, fn(s: &mut BotState, msg: &Message) -> String)> {
    let mut commands: Vec<(Regex, fn(s: &mut BotState, msg: &Message) -> String)> = Vec::new();
    commands.push((Regex::new(r"^!test").unwrap(), cmd_test));
    commands.push((Regex::new(r"^!save ").unwrap(), cmd_save_resource));
    commands.push((Regex::new(r"^!search ").unwrap(), cmd_search_resource));
    commands.push((Regex::new(r"^!status").unwrap(), cmd_status));

    return commands;
}

fn process_message(
    message: &Message,
    commands_array: &Vec<(Regex, fn(s: &mut BotState, msg: &Message) -> String)>,
    state: &mut BotState,
) {
    let content: &str = message.content.as_str();
    for (command, function) in commands_array {
        if command.is_match(content) {
            let response = function(state, message);

            state.last_command = Some(message.clone());

            if !response.is_empty() {
                let _ =
                    state
                        .discord
                        .send_message(message.channel_id, response.as_str(), "", false);
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

    // Test event
    let commands_array = commands_array();

    // State generation
    let mut bot_state = BotState {
        // Log in to Discord using a bot token from the environment
        discord: Discord::from_bot_token(&token).expect("login failed"),
        // Database
        db: DiscordDatabase::new(database_uri),
        last_command: None,
        last_command_output: "".to_string(),
    };

    // Establish and use a websocket connection
    let (mut connection, _) = bot_state.discord.connect().expect("connect failed");

    // Event loop
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                process_message(&message, &commands_array, &mut bot_state);
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

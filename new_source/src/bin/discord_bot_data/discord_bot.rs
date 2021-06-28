extern crate diesel;

pub mod bot_state;
pub mod commands;
pub mod utils;

use awesome_discord_bot::get_connection_manager;
use bot_state::DiscordHandler;
use diesel::r2d2::Pool;
use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serenity::http::Http;
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let connection = get_connection_manager();
    let pool = Pool::builder()
        .build(connection)
        .expect("Failed to create pool.");

    let handler = DiscordHandler { database: pool };

    let mut client = Client::builder(&token)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

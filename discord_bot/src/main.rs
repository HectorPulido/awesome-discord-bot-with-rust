mod emojis_only;
mod nlu;
mod resources;
mod rpg;
mod utils;

use emojis_only::emojis_channel_manager::manage_emojis_channel;
use nlu::nlu_manager::manage_mentions;
use resources::resource_manager::{manage_memes, manage_resources};

use serde_json::{Error as serdeError, Value};
use std::env;
use std::{collections::HashMap, sync::Arc};

use serenity::{
    async_trait,
    http::Http,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use utils::{add_reaction, mentions_me, Handler};

impl Handler {
    #[allow(dead_code)]
    pub async fn add_thumbs_down(context: &Context, message: &Message) {
        let unicode_reaction = String::from("👎");
        let _ = add_reaction(unicode_reaction, context, message).await;
    }

    pub async fn add_thumbs_up(context: &Context, message: &Message) {
        let unicode_reaction = String::from("👍");
        let _ = add_reaction(unicode_reaction, context, message).await;
    }

    pub fn get_credentials(&self) -> HashMap<&str, &str> {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("name", &self.name);
        map.insert("private_key", &self.key);
        return map;
    }

    pub async fn get_initial_config(&mut self) {
        let endpoint = format!("{}/initial-config/", self.endpoint);
        let map = self.get_credentials();
        let channel_data = self.client.post(endpoint).json(&map).send().await;

        if let Err(err) = channel_data {
            println!("Initial config connection failed {}", err);
            return;
        }
        let channel_data = channel_data.unwrap().text().await;

        if let Err(err) = channel_data {
            println!("Initial config load failed {}", err);
            return;
        }
        let channel_data = channel_data.unwrap();
        let channel_data: Result<Value, serdeError> = serde_json::from_str(&channel_data);

        if let Err(err) = channel_data {
            println!("Initial config format invalid {}", err);
            return;
        }

        self.channel_data = Some(channel_data.unwrap());
    }

    async fn add_msg_to_history(
        &self,
        ctx: &Context,
        prefix: String,
        message: &String,
        channel: String,
    ) {
        let history_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<History>()
                .expect("Expected CommandCounter in TypeMap.")
                .clone()
        };

        {
            let mut history = history_lock.write().await;
            let history_channel = history.entry(channel).or_insert(Vec::new());
            history_channel.push(format!("{}: {}", prefix, message));

            if history_channel.len() > 5 {
                history_channel.remove(0);
            }
        }
    }

    async fn show_msg_history(&self, ctx: &Context, msg: &Message) -> String {
        let history_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<History>()
                .expect("Expected CommandCounter in TypeMap.")
                .clone()
        };
        let channel = msg.channel_id.0.to_string();

        let mut conversation = String::new();

        let history = history_lock.read().await;
        if !history.contains_key(&channel) {
            return conversation;
        }

        for line in &history[&channel] {
            conversation = format!("{}{}\n", conversation, line);
        }

        return conversation;
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if self.channel_data == None {
            println!("Initial config load failed, please restart the program");
            return;
        }
        if msg.author.bot {
            return;
        }

        let channel_data = self.channel_data.as_ref().unwrap();

        match &channel_data[msg.channel_id.0.to_string()] {
            Value::String(record_type) => match record_type as &str {
                "RS" | "JO" => manage_resources(&self, &ctx, &msg, record_type).await,
                "ME" => manage_memes(&self, &ctx, &msg, record_type).await,
                "EM" => manage_emojis_channel(&self, &ctx, &msg).await,
                _ => {}
            },
            _ => {}
        }

        if mentions_me(&self, &msg) {
            manage_mentions(self, &ctx, &msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

struct History;

impl TypeMapKey for History {
    type Value = Arc<RwLock<HashMap<String, Vec<String>>>>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let endpoint =
        env::var("BRAIN_ENDPOINT").expect("Expected a BRAIN_ENDPOINT in the environment");
    let key = env::var("BRAIN_KEY").expect("Expected a BRAIN_KEY in the environment");
    let name = env::var("BRAIN_NAME").expect("Expected a BRAIN_NAME in the environment");

    let client = reqwest::Client::new();

    let http = Http::new_with_token(&token);
    let (_owner_id, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => (info.owner.id, info.id),
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let mut mut_bot_handler = Handler {
        owner_id: _owner_id,
        bot_id: _bot_id,
        endpoint,
        key,
        name,
        client,
        channel_data: None,
    };

    mut_bot_handler.get_initial_config().await;

    let mut client = Client::builder(&token)
        .event_handler(mut_bot_handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<History>(Arc::new(RwLock::new(HashMap::default())));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

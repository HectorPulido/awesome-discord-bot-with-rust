mod utils;
mod webscrapper;

use serde_json::Value;
use std::env;
use urlencoding::encode;

use serenity::{
    async_trait,
    http::Http,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use utils::{
    add_thumbs_up, clean_text, get_credentials, get_links, mentions_me, post_process, Handler,
};
use webscrapper::get_metatags;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        println!("{}: {}", msg.author.name, msg.content);

        let endpoint = format!("{}records/", self.endpoint);
        match &self.channel_data[msg.channel_id.0.to_string()] {
            Value::String(record_type) => match record_type as &str {
                "RS" | "JO" => {
                    if msg.embeds.len() > 0 {
                        let empty = String::from("");
                        for embed in &msg.embeds {
                            let title = embed.title.as_ref().unwrap_or(&empty);
                            let description = embed.description.as_ref().unwrap_or(&empty);
                            let url = embed
                                .url
                                .as_ref()
                                .unwrap_or(&empty)
                                .to_string()
                                .to_lowercase();
                            let url = url.trim();
                            let description = format!("{} | {} | {}", title, description, url);

                            let mut map = get_credentials(self);
                            map.insert("data", url.to_string());
                            map.insert("record_index", description);
                            map.insert("record_type", String::from(record_type));
                            let data = self
                                .client
                                .post(&endpoint)
                                .json(&map)
                                .send()
                                .await
                                .unwrap()
                                .text()
                                .await
                                .unwrap();
                            println!("Response: {:?}", data);
                            add_thumbs_up(&ctx, &msg).await;
                        }
                    } else {
                        let links = get_links(&msg.content);
                        for link in links {
                            let meta = get_metatags(&link).await;
                            let mut map = get_credentials(self);
                            map.insert("data", link);
                            map.insert("record_index", meta);
                            map.insert("record_type", String::from(record_type));
                            let data = self
                                .client
                                .post(&endpoint)
                                .json(&map)
                                .send()
                                .await
                                .unwrap()
                                .text()
                                .await
                                .unwrap();
                            println!("Response: {:?}", data);
                            add_thumbs_up(&ctx, &msg).await;
                        }
                    }
                }
                "ME" => {
                    for attch in &msg.attachments {
                        let mut map = get_credentials(self);

                        let url: &str = &attch.proxy_url;
                        let url = String::from(url);

                        let filename: &str = &attch.filename;
                        let filename = String::from(filename);

                        map.insert("data", url);
                        map.insert("record_index", filename);
                        map.insert("record_type", String::from(record_type));
                        let data = self
                            .client
                            .post(&endpoint)
                            .json(&map)
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();

                        println!("Response: {:?}", data);
                        add_thumbs_up(&ctx, &msg).await;
                    }
                }
                _ => {}
            },
            Value::Null | _ => {}
        }

        if mentions_me(&self, &msg) {
            let content = clean_text(&msg.content);
            let content = encode(&content);
            let endpoint = format!("{}phrase/{}/", self.endpoint, content);
            let map = get_credentials(&self);

            let resp = self
                .client
                .post(endpoint)
                .json(&map)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            println!("Response: {:?}", resp);

            let resp = post_process(&resp, &msg);

            println!("Response: {:?}", resp);

            for phrase in resp {
                if let Err(why) = msg.channel_id.say(&ctx.http, phrase).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let endpoint = env::var("BRAIN_ENDPOINT").expect("Expected a token in the environment");
    let key = env::var("BRAIN_KEY").expect("Expected a token in the environment");
    let name = env::var("BRAIN_NAME").expect("Expected a token in the environment");

    let channel_data = env::var("CONFIG_CHANNELS").expect("Expected a token in the environment");
    let channel_data: Value = serde_json::from_str(&channel_data).unwrap();

    let client = reqwest::Client::new();

    let http = Http::new_with_token(&token);
    let (_owner_id, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => (info.owner.id, info.id),
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let mut_bot_handler = Handler {
        owner_id: _owner_id,
        bot_id: _bot_id,
        endpoint: endpoint,
        key: key,
        name: name,
        client: client,
        channel_data: channel_data,
    };

    let mut client = Client::builder(&token)
        .event_handler(mut_bot_handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

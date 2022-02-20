use super::webscrapper::{get_links, get_metatags};
use crate::utils::Handler;
use serenity::{model::channel::Message, prelude::*};

async fn save_and_show(
    handler: &Handler,
    ctx: &Context,
    msg: &Message,
    link: &str,
    meta: &str,
    record_type: &str,
) {
    let endpoint = format!("{}/records/", handler.endpoint);
    let mut map = handler.get_credentials();
    map.insert("data", link);
    map.insert("record_index", meta);
    map.insert("record_type", record_type);
    handler
        .client
        .post(endpoint)
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    Handler::add_thumbs_up(&ctx, &msg).await;
}

pub async fn manage_resources(handler: &Handler, ctx: &Context, msg: &Message, record_type: &str) {
    if msg.embeds.len() > 0 {
        let empty = String::from("");
        for embed in &msg.embeds {
            let title = embed.title.as_ref().unwrap_or(&empty);
            let description = embed.description.as_ref().unwrap_or(&empty);
            let link = embed.url.as_ref().unwrap_or(&empty).to_string();
            let link = link.trim();
            let meta = format!("{} | {} | {}", title, description, link);

            save_and_show(handler, ctx, msg, &link, &meta, record_type).await;
        }
    } else {
        let links = get_links(&msg.content);
        for link in links {
            let meta = get_metatags(&link).await;

            save_and_show(handler, ctx, msg, &link, &meta, record_type).await;
        }
    }
}

pub async fn manage_memes(handler: &Handler, ctx: &Context, msg: &Message, record_type: &str) {
    for attch in &msg.attachments {
        let link: &str = &attch.proxy_url;
        let meta: &str = &attch.filename;

        save_and_show(handler, ctx, msg, link, meta, record_type).await;
    }
}

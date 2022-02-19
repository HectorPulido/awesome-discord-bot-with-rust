use super::super::utils::Handler;
use super::webscrapper::{get_links, get_metatags};
use serenity::{model::channel::Message, prelude::*};

pub async fn manage_resources(handler: &Handler, ctx: &Context, msg: &Message, record_type: &str) {
    if msg.embeds.len() > 0 {
        let empty = String::from("");
        for embed in &msg.embeds {
            let title = embed.title.as_ref().unwrap_or(&empty);
            let description = embed.description.as_ref().unwrap_or(&empty);
            let link = embed.url.as_ref().unwrap_or(&empty).to_string();
            let link = link.trim();
            let meta = format!("{} | {} | {}", title, description, link);

            handler
                .save_and_show(ctx, msg, &link, &meta, record_type)
                .await;
        }
    } else {
        let links = get_links(&msg.content);
        for link in links {
            let meta = get_metatags(&link).await;
            handler
                .save_and_show(ctx, msg, &link, &meta, record_type)
                .await;
        }
    }
}

pub async fn manage_memes(handler: &Handler, ctx: &Context, msg: &Message, record_type: &str) {
    for attch in &msg.attachments {
        let link: &str = &attch.proxy_url;
        let meta: &str = &attch.filename;

        handler
            .save_and_show(ctx, msg, link, meta, record_type)
            .await;
    }
}

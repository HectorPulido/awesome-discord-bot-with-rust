use super::super::utils::Handler;
use super::utils::*;
use serenity::{model::channel::Message, prelude::*};

pub async fn manage_mentions(handler: &Handler, ctx: &Context, msg: &Message) {
    let content = clean_text(&msg.content);

    handler
        .add_msg_to_history(
            &ctx,
            "H".to_string(),
            &content,
            msg.channel_id.0.to_string(),
        )
        .await;

    let history = handler.show_msg_history(ctx, msg).await;

    let endpoint = format!("{}phrase/", handler.endpoint);

    let mut map = handler.get_credentials();
    map.insert("query", &content);
    map.insert("history", &history);

    let resp = handler
        .client
        .post(endpoint)
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let resp = post_process(&resp, &msg);

    for phrase in resp {
        let clean_phrase = clean_text(&phrase);
        let clean_phrase = remove_links(&clean_phrase);
        handler
            .add_msg_to_history(
                &ctx,
                "P".to_string(),
                &clean_phrase,
                msg.channel_id.0.to_string(),
            )
            .await;
        if let Err(why) = msg.channel_id.say(&ctx.http, phrase).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

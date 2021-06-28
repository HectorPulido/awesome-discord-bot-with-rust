extern crate awesome_discord_bot;
use super::bot_state::DiscordHandler;
use super::utils::*;
use awesome_discord_bot::models::resources::Resources;
use awesome_discord_bot::models::types::Types;
use regex::Regex;
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[allow(dead_code)]
pub async fn help(_: &DiscordHandler, ctx: &Context, msg: &Message) {
    let author = &msg.author.name;
    let response = format!("Has dicho hola {}", author);

    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        println!("[Error] sending message: {:?}", why);
    }
}

#[allow(dead_code)]
pub async fn get_types(handler: &DiscordHandler, ctx: &Context, msg: &Message) {
    let db = handler.database.get().expect("could not get db connection");
    let mention = get_mention(msg);

    let message: String = match Types::get_types(&db) {
        Ok(types) => {
            if types.len() == 0 {
                format!("{} No hay ningun tipo aun", mention)
            } else {
                let mut message = String::from("Los tipos de canales son:");
                for type_data in types {
                    message = format!("{} \"{}\"", message, type_data.type_description);
                }
                message
            }
        }
        Err(_) => {
            format!(
                "{} Ha ocurrido un error, contacta con el administrador",
                mention
            )
        }
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        println!("[Error] sending message: {:?}", why);
    }
    return;
}

#[allow(dead_code)]
pub async fn search_resource(handler: &DiscordHandler, ctx: &Context, msg: &Message) {
    let mention = get_mention(msg);
    let db = handler.database.get().expect("could not get db connection");
    let input = msg.content.as_str();
    let re_query: Regex = Regex::new(r"!search (?P<query>.*)").unwrap();

    // TODO validar canal, validar tipo

    let query = re_query
        .captures(input)
        .and_then(|cap| cap.name("query").map(|m| m.as_str()))
        .unwrap();

    let query = format!("%{}%", query).to_string();
    let results = Resources::get_random_resource(&query, &db);

    match results {
        Ok(response) => {
            if response.len() > 0 {
                let message = format!(
                    "He encontrado eso {} espero que te sirva {}",
                    mention, response[0].url
                ); // TODO: embed

                if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                    println!("[Error] sending message: {:?}", why);
                }

                return;
            } else {
                let message = format!(
                    "No he podido encontrar nada relacionado {} \
                    por favor prueba con otras palabras claves",
                    mention
                );

                if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                    println!("[Error] sending message: {:?}", why);
                }
                return;
            }
        }
        Err(_) => {
            let message = format!(
                "He tenido un problema al buscar, contacta con el administrador {}",
                mention
            );

            if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                println!("[Error] sending message: {:?}", why);
            }
            return;
        }
    };
}

#[allow(dead_code)]
pub async fn save_resource(handler: &DiscordHandler, ctx: &Context, msg: &Message) {
    let db = handler.database.get().expect("could not get db connection");
    let mention = get_mention(msg);
    let author_id = &msg.author.id;
    let channel_id = &msg.channel_id.0;
    let embeds = &msg.embeds;

    let mut save_result = true;
    let empty = String::from("");
    for embed in embeds {
        let title = embed.title.as_ref().unwrap_or(&empty);
        let description = embed.description.as_ref().unwrap_or(&empty);
        let url = embed
            .url
            .as_ref()
            .unwrap_or(&empty)
            .to_string()
            .to_lowercase();
        let url = url.trim();
        let mut description = format!("{} {} {}", url, title, description);

        description = description.to_lowercase().trim().to_string();

        if url.is_empty() || description.is_empty() {
            save_result = false;
            continue;
        }

        match Resources::create_resource(
            &db,
            author_id.to_string(),
            channel_id.to_string(),
            url.to_string(),
            description.to_string(),
        ) {
            Ok(_) => (),
            Err(err) => {
                save_result = false;
                println!("{}", err);
            }
        }
    }
    if save_result {
        add_thumbs_up(ctx, msg).await;
    } else {
        add_thumbs_down(ctx, msg).await;
    }
    let info = format!(
        "No se pudo salvar la informacion de el mensaje de {}",
        mention
    );

    if let Err(why) = msg.channel_id.say(&ctx.http, info).await {
        println!("[Error] sending message: {:?}", why);
    }
}

#[allow(dead_code)]
pub async fn save_channel(_: &DiscordHandler, ctx: &Context, msg: &Message) {
    let is_admin = is_admin(ctx, msg).await;
    println!("is_admin {}", is_admin);
    // let guild = ctx.http.get_guild_roles(msg.channel_id.0)
}

// async fn am_i_admin(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
//     if let Some(member) = &msg.member {

//         for role in &member.roles {
//             if role.to_role_cached(&ctx.cache).await.map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR)) {
//                 msg.channel_id.say(&ctx.http, "Yes, you are.").await?;

//                 return Ok(());
//             }
//         }
//     }

//     msg.channel_id.say(&ctx.http, "No, you are not.").await?;

//     Ok(())
// }

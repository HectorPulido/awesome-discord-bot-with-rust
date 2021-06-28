use super::commands::*;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use regex::Regex;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct DiscordHandler {
    pub database: Pool<ConnectionManager<PgConnection>>,
}

#[allow(unused_macros)]
macro_rules! command {
    ($func:ident, $re:expr, $msg:expr, $ctx:expr, $handler:expr) => {
        let re: Regex = Regex::new($re).unwrap();
        if re.is_match(&$msg.content) {
            $func(&$handler, &$ctx, &$msg).await;
            return;
        }
    };
}

#[async_trait]
impl EventHandler for DiscordHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        command!(get_types, r"!types", msg, ctx, self);
        command!(help, r"!help", msg, ctx, self);
        command!(save_resource, r"!save ", msg, ctx, self);
        command!(search_resource, r"!search ", msg, ctx, self);
        command!(save_channel, r"!save_channel ", msg, ctx, self);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

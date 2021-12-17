#![allow(dead_code)]

use std::sync::{Arc};
use serenity::{
    async_trait,
    Client,
    model::{
        prelude::{
            Interaction,
            Ready,
        }
    },
    prelude::{Context, EventHandler}
};
use sqlx::SqlitePool;
use crate::cache::Caches;

use crate::config::{Config, read_config};
use crate::token::read_token;

mod token;
mod config;
mod command;
mod cache;

#[tokio::main]
async fn main() {
    let token = read_token();
    let config = Arc::new(read_config());
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(&config.db_path)
                .create_if_missing(true),
        )
        .await
        .expect("Error while db connection");

    sqlx::migrate!("./migrations").run(&db)
        .await
        .expect("Couldn't run database migrations");

    let caches = Caches::new();


    let mut client = Client::builder(token)
        .application_id(config.application_id)
        .event_handler(Handler {
            db,
            caches,
            config,
        })
        .await
        .expect("Error while creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}


struct Handler {
    db: SqlitePool,
    caches: Caches,
    config: Arc<Config>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    }
}
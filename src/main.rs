#![allow(dead_code)]

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
use crate::token::read_token;
mod token;
mod config;
mod command;
mod cache;

#[tokio::main]
async fn main() {
    let token = read_token();

    let mut client = Client::builder(token)
        .application_id(config.application_id)
        .event_handler(Handler)
        .await
        .expect("Error while creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    }
}
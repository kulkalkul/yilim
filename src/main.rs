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
use crate::command::CommandsHandler;

use crate::commands::{
    set_guidelines_channel_fn,
    set_help_channel_fn,
    add_help_category_fn,
    HELP_BUTTON_CLICK_ID,
    set_help_log_answered_channel_fn,
    topic_fn,
    set_help_log_waiting_channel_fn
};

use crate::config::{Config, read_config};
use crate::interactions::{
    send_categories_fn,
    send_channels_fn,
    send_button_create_fn,
    create_help_thread_fn,
    HELP_SELECT_CATEGORY_ID,
    HELP_SELECT_CHANNEL_ID,
    HELP_CREATE_CLICK_ID,
};
use crate::token::read_token;

mod token;
mod config;
mod command;
mod commands;
mod interactions;
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

    let commands_handler = CommandsHandler::new(
        config.clone(),
        db.clone(),
        caches.clone(),
        )
        .add_template_command("set_guidelines_channel", set_guidelines_channel_fn)
        .add_template_command("set_help_log_answered_channel", set_help_log_answered_channel_fn)
        .add_template_command("set_help_log_waiting_channel", set_help_log_waiting_channel_fn)
        .add_command("set_help_channel", set_help_channel_fn)
        .add_command("add_help_category", add_help_category_fn)
        .add_command("topic", topic_fn);

    let mut client = Client::builder(token)
        .application_id(config.application_id)
        .event_handler(Handler {
            db,
            caches,
            config,
            commands_handler,
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
    commands_handler: CommandsHandler,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild_status in ready.guilds {
            let mut guild = guild_status.id();
            let commands = guild
                .set_application_commands(&ctx.http, |commands| {
                    self.commands_handler.register_commands(commands);
                    commands
                })
                .await
                .unwrap_or_else(|e| {
                    panic!("Error while registering commands for guild {}: {}", guild.0, e);
                });

            self.commands_handler
                .register_permissions(&mut guild, &ctx.http, commands)
                .await;
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            self.commands_handler.handle_response(&command, &ctx).await;
        } else if let Interaction::MessageComponent(component) = interaction {
            match component.data.custom_id.as_str() {
                HELP_BUTTON_CLICK_ID => send_categories_fn(&component, &ctx, &self.db)
                    .await,
                HELP_SELECT_CATEGORY_ID => send_channels_fn(&component, &ctx)
                    .await,
                HELP_SELECT_CHANNEL_ID => send_button_create_fn(&component, &ctx, &self.caches)
                    .await,
                HELP_CREATE_CLICK_ID => create_help_thread_fn(&component, &ctx, &self.caches)
                    .await,
                _ => (),
            }
        }
    }
}
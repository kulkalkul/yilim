use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::application_command::ApplicationCommandPermissionType;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use sqlx::SqlitePool;
use crate::command::{Command, Response};
use crate::config::Config;

pub const GUIDELINES_CHANNEL: &'static str = "guidelines";

const CHANNEL_OPTION: &'static str = "channel";

pub fn set_guidelines_channel_fn(mut command: Command) -> Command {
    let Config {administrator_id, .. } = *command.config;

    command
        .description("Yönergeler kanalını ayarlar.")
        .default_permission(false)
        .create_option(|option| {
            option
                .name(CHANNEL_OPTION)
                .kind(ApplicationCommandOptionType::Channel)
                .required(true)
                .description("Ayarlanacak yönergeler kanalı.")
        })
        .set_permissions(move |permissions| {
            permissions
                .create_permission(|permission| {
                    permission
                        .id(administrator_id)
                        .kind(ApplicationCommandPermissionType::User)
                        .permission(true)
                })
        })
        .set_response(|mut ctx| async move {
            let channel = ctx.options.read_channel(CHANNEL_OPTION)
                .await
                .unwrap();

            insert_guidelines_channel(&ctx.db, channel.id.0 as i64)
                .await;

            Response::message(move |message| {
                message
                    .content(format!("\
                        Yönergeler kanalı başarıyla <#{}> olarak ayarlandı.\
                    ", channel.id))
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
        });

    command
}

async fn insert_guidelines_channel(db: &SqlitePool, id: i64) {
    sqlx::query!(
        "INSERT OR REPLACE INTO channels (name, id) VALUES (?, ?)",
        GUIDELINES_CHANNEL,
        id,
    )
        .execute(db)
        .await
        .unwrap();
}
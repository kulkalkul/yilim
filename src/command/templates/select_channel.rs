use serenity::model::prelude::application_command::{ApplicationCommandOptionType, ApplicationCommandPermissionType};
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use sqlx::SqlitePool;
use crate::command::{Command, Response};
use crate::command::template::Template;
use crate::Config;

const CHANNEL_OPTION: &'static str = "channel";

pub struct SelectChannelTemplate {
    pub channel_name: &'static str,
    pub channel_display_name: &'static str,
    pub uppercase_channel_display_name: String,
    pub save_to_database: bool,
}

impl SelectChannelTemplate {
    pub fn new(
        channel_name: &'static str,
        channel_display_name: &'static str,
        save_to_database: bool,
    ) -> Self {
        let uppercase_channel_display_name = {
            let mut chars = channel_display_name.chars();
            match chars.next() {
                None => String::new(),
                Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
            }
        };

        Self {
            channel_name,
            channel_display_name,
            uppercase_channel_display_name,
            save_to_database,
        }
    }
}

impl Template for SelectChannelTemplate {
    fn run(&self, mut command: Command) -> Command {
        let Config { administrator_id, .. } = *command.config;

        let SelectChannelTemplate {
            channel_name,
            channel_display_name,
            save_to_database,
            ..
        } = *self;
        let uppercase_channel_display_name = self.uppercase_channel_display_name.clone();

        command
            .description(format!("{} kanalını seçer.", uppercase_channel_display_name))
            .default_permission(false)
            .create_option(|option| {
                option
                    .name(CHANNEL_OPTION)
                    .kind(ApplicationCommandOptionType::Channel)
                    .required(true)
                    .description(format!("Seçilecek {} kanalı.", channel_display_name))
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
            .set_response(move |mut ctx| {
                let uppercase_channel_name = uppercase_channel_display_name.clone();

                async move {
                    let channel = ctx.options.read_channel(CHANNEL_OPTION)
                        .await
                        .unwrap();

                    if save_to_database {
                        insert_channel(&ctx.db, channel_name, channel.id.0 as i64)
                            .await;
                    }

                    Response::message(move |message| {
                        message
                            .content(format!(
                                "{} kanalı başarıyla <#{}> olarak seçildi.",
                                uppercase_channel_name,
                                channel.id
                            ))
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    })
                }
            });

        command
    }
}

async fn insert_channel(db: &SqlitePool, name: &'static str, id: i64) {
    sqlx::query!(
        "INSERT OR REPLACE INTO channels (name, id) VALUES (?, ?)",
        name,
        id,
    )
        .execute(db)
        .await
        .unwrap();
}
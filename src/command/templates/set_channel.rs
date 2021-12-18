use serenity::model::prelude::application_command::{ApplicationCommandOptionType, ApplicationCommandPermissionType};
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use sqlx::SqlitePool;
use crate::command::{Command, Response};
use crate::command::template::Template;
use crate::Config;

const CHANNEL_OPTION: &'static str = "channel";

pub struct SetChannelTemplate {
    channel_name: &'static str,
    channel_id: &'static str,
    uppercase_channel_name: String,
}

impl SetChannelTemplate {
    pub fn new(channel_name: &'static str, channel_id: &'static str) -> Self {
        let uppercase_channel_name = {
            let mut chars = channel_name.chars();
            match chars.next() {
                None => String::new(),
                Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
            }
        };

        Self {
            channel_name,
            channel_id,
            uppercase_channel_name,
        }
    }
}

impl Template for SetChannelTemplate {
    fn run(&self, mut command: Command) -> Command {
        let Config { administrator_id, .. } = *command.config;

        let channel_name = self.channel_name;
        let channel_id = self.channel_id;
        let uppercase_channel_name = self.uppercase_channel_name.clone();

        command
            .description(format!("{} kanalını ayarlar.", uppercase_channel_name))
            .default_permission(false)
            .create_option(|option| {
                option
                    .name(CHANNEL_OPTION)
                    .kind(ApplicationCommandOptionType::Channel)
                    .required(true)
                    .description(format!("Ayarlanacak {} kanalı.", channel_name))
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
                let uppercase_channel_name = uppercase_channel_name.clone();

                async move {
                    let channel = ctx.options.read_channel(CHANNEL_OPTION)
                        .await
                        .unwrap();

                    insert_channel(channel_id, &ctx.db, channel.id.0 as i64)
                        .await;

                    Response::message(move |message| {
                        message
                            .content(format!(
                                "{} kanalı başarıyla <#{}> olarak ayarlandı.",
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

async fn insert_channel(channel_name: &'static str, db: &SqlitePool, id: i64) {
    sqlx::query!(
        "INSERT OR REPLACE INTO channels (name, id) VALUES (?, ?)",
        channel_name,
        id,
    )
        .execute(db)
        .await
        .unwrap();
}
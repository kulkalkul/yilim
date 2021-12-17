use serenity::model::channel::ChannelType;
use serenity::model::interactions::application_command::{ApplicationCommandOptionType, ApplicationCommandPermissionType};
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use sqlx::SqlitePool;
use crate::command::{Command, Response};
use crate::config::Config;

const CATEGORY_OPTION: &'static str = "category";

pub fn add_help_category_fn(mut command: Command) -> Command {
    let Config {administrator_id, .. } = *command.config;

    command
        .description("Yardım kategorisi ekler veya çıkartır.")
        .default_permission(false)
        .create_option(|option| {
            option
                .name(CATEGORY_OPTION)
                .kind(ApplicationCommandOptionType::Channel)
                .required(true)
                .description("Eklenecek veya çıkartılacak yardım kategorisi.")
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
            let category = ctx.options.read_channel(CATEGORY_OPTION)
                .await
                .unwrap();

            if category.kind != ChannelType::Category {
                return Response::message(move |message| {
                    message
                        .content(format!("\
                            <#{}> bir kategori değil, lütfen bir kategori seçiniz.\
                        ", category.id))
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
            }

            let exists = read_help_category_exists(&ctx.db, category.id.0 as i64)
                .await;

            let response = if exists {
                remove_help_category(&ctx.db, category.id.0 as i64)
                    .await;
                format!("<#{}> yardım kategorileri arasından çıkartıldı.", category.id)
            } else {
                let name = category.name.replace("Yardımlaşma: ", "");
                insert_help_category(&ctx.db, category.id.0 as i64, name)
                    .await;

                format!("<#{}> yardım kategorileri arasına eklendi.", category.id)
            };

            Response::message(move |message| {
                message
                    .content(response)
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
        });

    command
}

async fn read_help_category_exists(db: &SqlitePool, id: i64) -> bool {
    let exists = sqlx::query!(
        r#"SELECT EXISTS(SELECT 1 FROM help_categories WHERE id = ?) as "exists""#,
        id,
    )
        .fetch_one(db)
        .await
        .unwrap()
        .exists;

    exists == 1
}

async fn insert_help_category(db: &SqlitePool, id: i64, name: String) {
    sqlx::query!(
        "INSERT OR REPLACE INTO help_categories (id, name) VALUES (?, ?)",
        id,
        name,
    )
        .execute(db)
        .await
        .unwrap();
}

async fn remove_help_category(db: &SqlitePool, id: i64) {
    sqlx::query!(
        "DELETE FROM help_categories WHERE id = ?",
        id,
    )
        .execute(db)
        .await
        .unwrap();
}
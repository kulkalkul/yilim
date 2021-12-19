use serenity::model::prelude::message_component::ButtonStyle;
use sqlx::SqlitePool;
use crate::command::{Command, Template};
use crate::command::templates::SelectChannelTemplate;

const CHANNEL_OPTION: &'static str = "channel";

pub struct SendRoleToggleMessage {
    pub channel_message: &'static str,
    pub role_id: u64,
    pub interaction_id: &'static str,
    pub select_channel: SelectChannelTemplate,
}

impl SendRoleToggleMessage {
    pub fn new(
        channel_message: &'static str,
        role_id: u64,
        interaction_id: &'static str,
        channel_name: &'static str,
        channel_display_name: &'static str
    ) -> Self {
        Self {
            channel_message,
            role_id,
            interaction_id,
            select_channel: SelectChannelTemplate::new(channel_name, channel_display_name, false),
        }
    }
}

impl Template for SendRoleToggleMessage {
    fn run(&self, command: Command) -> Command {
        let mut command = self.select_channel.run(command);

        let response_fn = command.response_fn
            .take()
            .unwrap();

        let SendRoleToggleMessage {
            channel_message,
            interaction_id,
            ..
        } = *self;

        let SelectChannelTemplate {
            channel_name,
            ..
        } = self.select_channel;

        command.set_response(move |mut ctx| {
            let response = (response_fn)(ctx.clone());

            async move {
                let channel = ctx.options
                    .read_channel(CHANNEL_OPTION)
                    .await
                    .unwrap();

                if let Ok(record) = read_toggle_message(&ctx.db, channel_name).await {
                    ctx.http
                        .delete_message(record.channel_id as u64, record.id as u64)
                        .await
                        .ok();
                }

                let message = channel.id.send_message(&ctx.http, |message| {
                    message
                        .content(channel_message)
                        .components(|components| {
                            components.create_action_row(|row| {
                                row.create_button(|button| {
                                    button
                                        .custom_id(interaction_id)
                                        .label("Değiştir")
                                        .style(ButtonStyle::Primary)
                                })
                            })
                        })
                })
                    .await
                    .expect("Error while sending message");

                insert_role_message(
                    &ctx.db,
                    channel_name,
                    message.channel_id.0 as i64,
                    message.id.0 as i64,
                )
                    .await;

                response.await
            }

        });

        command
    }
}

async fn insert_role_message(db: &SqlitePool, name: &'static str, channel_id: i64, message_id: i64) {
    sqlx::query!(
        "INSERT OR REPLACE INTO messages (name, id, channel_id) VALUES (?, ?, ?)",
        name,
        message_id,
        channel_id,
    )
        .execute(db)
        .await
        .unwrap();
}

struct MessageRecord {
    channel_id: i64,
    id: i64,
}

async fn read_toggle_message(db: &SqlitePool, name: &'static str) -> Result<MessageRecord, sqlx::Error> {
    sqlx::query_as!(
        MessageRecord,
        "SELECT id, channel_id FROM messages WHERE name = ?",
        name,
    )
        .fetch_one(db)
        .await
}

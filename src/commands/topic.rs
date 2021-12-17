use std::ops::{RangeInclusive};
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::{ChannelType, InteractionApplicationCommandCallbackDataFlags};
use crate::command::{Command, Response};

const TEXT_OPTION: &'static str = "text";
const TOPIC_RANGE: RangeInclusive<usize> = 2..=100;

pub fn topic_fn(mut command: Command) -> Command {
    command
        .description("Soru başlığı değiştirir.")
        .create_option(|option| {
            option
                .name(TEXT_OPTION)
                .kind(ApplicationCommandOptionType::String)
                .required(true)
                .description("Konulacak başlık. En az 2, en fazla 100 harf olabilir.")
        })
        .set_response(|mut ctx| async move {
            let mut thread = ctx.channel_id
                .to_channel(&ctx.http)
                .await
                .unwrap()
                .guild()
                .unwrap();

            if thread.kind != ChannelType::PublicThread && thread.kind != ChannelType::PrivateThread {
                return Response::message(|message| {
                    message
                        .content("Bu komutu sadece size ait sorular içinde tek sefaya mahsus kullanabilirsiniz.")
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
            }

            let message = ctx.caches.help_creation_messages
                .get_message(thread.id.0, ctx.user.id.0);

            if message == None {
                return Response::message(|message| {
                    message
                        .content("Bu komutu sadece size ait sorular içinde tek sefaya mahsus kullanabilirsiniz.")
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
            }

            let topic = ctx.options.read_string(TEXT_OPTION)
                .await
                .unwrap();

            if !TOPIC_RANGE.contains(&topic.len()) {
                return Response::message(|message| {
                    message
                        .content("Başlığınız çok kısa veya çok uzun, 2-100 harf arasında olmalı.")
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
            }

            ctx.caches.help_creation_messages.remove(thread.id.0);
            let (channel, message) = message.unwrap();

            channel.edit_message(&ctx.http, message, |edit| {
                edit
                    .content(format!("<@{}> bir soru sordu: **{}**", ctx.user.id.0, topic.clone()))
            })
                .await
                .expect("");

            thread.edit(&ctx.http, |channel| {
                channel
                    .name(topic.clone())
            })
                .await
                .expect("");

            Response::message(move |message| {
                message
                    .content(format!("\
                        Başlık ``{}`` olarak değiştirildi.\
                    ", topic))
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
        });

    command
}
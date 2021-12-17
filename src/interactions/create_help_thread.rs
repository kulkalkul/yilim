use std::time::{Duration, Instant};
use serenity::model::channel::ChannelType;
use serenity::model::prelude::{InteractionApplicationCommandCallbackDataFlags, InteractionResponseType};
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::prelude::Context;
use crate::Caches;

const BUTTON_COOLDOWN: Duration = Duration::from_secs(60 * 10);

pub async fn create_help_thread_fn(
    component: &MessageComponentInteraction,
    ctx: &Context,
    caches: &Caches,
) {
    let user_id = component.user.id.0;
    let channel = caches.selected_channels.get(user_id);

    if channel == None {
        component.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|interaction| {
                    interaction
                        .content("\
                            Kanal kaydınızı daha önce yaptığınız için cache'ten silinmiş, kanalı \
                            tekrar seçiniz veya süreci tekradan başlatınız.\
                        ")
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
        })
            .await
            .expect("");
        return;
    }

    let since = caches.cooldowns.since(user_id);

    if let Some(difference) = since {
        if difference < BUTTON_COOLDOWN {
            component.create_interaction_response(&ctx.http, |response| {
                let remaining = (BUTTON_COOLDOWN - difference).as_secs() + 1;
                let minutes = remaining / 60;
                let seconds = remaining % 60;

                let time_string = if minutes == 0 {
                    format!("{} saniye", seconds)
                } else if seconds == 0 {
                    format!("{} dakika", minutes)
                } else {
                    format!("{} dakika {} saniye", minutes, seconds)
                };

                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|interaction| {
                        interaction
                            .content(format!("\
                                    Sıradaki sorunuzu {} sonra sorabilirsiniz.\
                                ", time_string))
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    })

            })
                .await
                .expect("");

            return;
        }
    }

    component.create_interaction_response(&ctx.http, |response| {
        caches.cooldowns.put(user_id, Instant::now());

        response
            .kind(InteractionResponseType::DeferredChannelMessageWithSource)
            .interaction_response_data(|interaction| {
                interaction
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
    })
        .await
        .expect("");

    let channel = channel.unwrap();

    let message = channel
        .send_message(&ctx.http, |message| {
            message.content(format!("\
                **<@{}>, şu anda sorunuzun bir başlığı yok, sorunuza ``/topic`` komutunu kullanarak \
                başlık ekleyebilirsiniz. Başlık eklediğinizde bu mesaj başlığa dönüşecektir.**\
            ", user_id))
        })
        .await
        .unwrap()
        .id;

    let thread = channel.create_public_thread(&ctx.http, message.0, |thread| {
        thread
            .name("Başlıksız Soru")
            .kind(ChannelType::PublicThread)
    })
        .await
        .expect("");

    caches.help_creation_messages.put(thread.id.0, user_id, channel.0, message.0);

    component.create_followup_message(&ctx.http, |response| {
        response
            .content(format!("Sorunuz oluşturuldu: {}", thread))
            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
    })
        .await
        .expect("");
}
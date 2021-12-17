use serenity::model::prelude::{InteractionApplicationCommandCallbackDataFlags, InteractionResponseType};
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::prelude::Context;

pub const HELP_SELECT_CHANNEL_ID: &'static str = "help_select_channel";

pub async fn send_channels_fn(component: &MessageComponentInteraction, ctx: &Context) {
    let id: u64 = component.data.values
        .first()
        .unwrap()
        .parse()
        .unwrap();

    let guild_id = ctx.cache
        .channel(id)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .guild_id;

    let all_channels = ctx.http
        .get_channels(guild_id.0)
        .await
        .unwrap();

    let mut channels = Vec::new();

    for channel in all_channels {
        if let Some(category_id) = channel.category_id {
            if category_id == id {
                channels.push(channel);
            }
        }
    }

    component.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|interaction| {
                interaction
                    .content("\
                        Sormak istediğiniz kanalı seçiniz.
                    ")
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    .components(|components| {
                        components.create_action_row(|row| {
                            row.create_select_menu(|select| {
                                select
                                    .custom_id(HELP_SELECT_CHANNEL_ID)
                                    .min_values(1)
                                    .max_values(1)
                                    .placeholder("Sormak istediğiniz kanal")
                                    .options(|options| {
                                        for channel in channels {
                                            options.create_option(|option| {
                                                option
                                                    .label(channel.name)
                                                    .value(channel.id.0)
                                            });
                                        }

                                        options
                                    })
                            })
                        })
                    })

            })
    })
        .await
        .expect("");
}
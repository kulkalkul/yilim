use serenity::model::interactions::InteractionResponseType;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::prelude::Context;
use crate::Caches;

pub const HELP_CREATE_CLICK_ID: &'static str = "help_create_click";

pub async fn send_button_create_fn(
    component: &MessageComponentInteraction,
    ctx: &Context,
    caches: &Caches,
) {
    let user_id = component.user.id.0;
    let channel_id: u64 = component.data.values
        .first()
        .unwrap()
        .parse()
        .unwrap();

    caches.selected_channels.put(user_id, channel_id);

    component.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|interaction| {
                interaction
                    .content(format!("\
                        Sorunuzu <#{}> kanalına sormak istediğinizden eminseniz aşağıdaki onaylama, \
                        butonuna tıklayınız. Sorunuzu yolladıktan sonra ``/topic`` komutunu kullanarak \
                        sorunuza açıklayıcı kısa bir başlık vermeyi unutmayın! Ayrıca, sadece 10 \
                        dakikada bir soru konusu açabilirsiniz.\
                    ", channel_id))
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    .components(|components| {
                        components
                            .create_action_row(|row| {
                                row.create_button(|button| {
                                    button
                                        .custom_id(HELP_CREATE_CLICK_ID)
                                        .label("Onaylıyorum, Sorumu Sor")
                                        .style(ButtonStyle::Danger)
                                })
                            })
                    })
            })
    })
        .await
        .expect("");
}

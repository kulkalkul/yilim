use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::model::prelude::{InteractionApplicationCommandCallbackDataFlags, RoleId};
use serenity::prelude::Context;
use crate::Config;

pub const TWITCH_TOGGLE_CLICK_ID: &'static str = "twitch_toggle";

pub async fn twitch_toggle_role_fn(
    component: &MessageComponentInteraction,
    ctx: &Context,
    config: &Config,
) {
    let mut member = component
        .member
        .clone()
        .unwrap();

    let role = RoleId(config.role_ids.twitch);
    let message = if member.roles.contains(&role) {
        member.remove_role(&ctx.http, config.role_ids.twitch)
            .await
            .unwrap();
        "Twitch yayını bildirimi aboneliğinizden **vazgeçtiniz**. Tekrar abone olmak isterseniz \
        aynı butonu kullanabilirsiniz."
    } else {
        member.add_role(&ctx.http, config.role_ids.twitch)
            .await
            .unwrap();
        "Twitch yayını bildirimlerine abone oldunuz, artık Twitch yayınlarını olduğunda **haberdar \
        edileceksiniz**. Aboneliğinizi iptal ettirmek isterseniz aynı butonu kullanabilirsiniz."
    };

    component.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|interaction| {
                interaction
                    .content(message)
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
    })
        .await
        .expect("");
}
use crate::command::TemplateContext;
use crate::command::templates::{SendRoleToggleMessage};


pub const TWITCH_TOGGLE_CHANNEL_INTERACTION_NAME: &'static str = "twitch_toggle";
pub const TWITCH_TOGGLE_CHANNEL_NAME: &'static str = "twitch_toggle";
pub const TWITCH_TOGGLE_CHANNEL_DISPLAY_NAME: &'static str = "twitch_toggle";

pub fn set_twitch_toggle_channel_fn(ctx: TemplateContext) -> SendRoleToggleMessage {
    SendRoleToggleMessage::new(
        "Twitch yayınlarının bildirimini Discord üzerinden almak istiyorum/istemiyorum.",
        ctx.config.role_ids.twitch,
        TWITCH_TOGGLE_CHANNEL_INTERACTION_NAME,
        TWITCH_TOGGLE_CHANNEL_NAME,
        TWITCH_TOGGLE_CHANNEL_DISPLAY_NAME,
    )
}
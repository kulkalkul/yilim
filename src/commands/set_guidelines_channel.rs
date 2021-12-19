use crate::command::templates::{ SetChannelTemplate };

pub const GUIDELINES_CHANNEL_NAME: &'static str = "guidelines";
pub const GUIDELINES_CHANNEL_DISPLAY_NAME: &'static str = "guidelines";

pub fn set_guidelines_channel_fn() -> SetChannelTemplate {
    SetChannelTemplate::new(
        GUIDELINES_CHANNEL_NAME,
        GUIDELINES_CHANNEL_DISPLAY_NAME,
    )
}

use crate::command::templates::{SelectChannelTemplate};

pub const GUIDELINES_CHANNEL_NAME: &'static str = "guidelines";
pub const GUIDELINES_CHANNEL_DISPLAY_NAME: &'static str = "guidelines";

pub fn set_guidelines_channel_fn() -> SelectChannelTemplate {
    SelectChannelTemplate::new(
        GUIDELINES_CHANNEL_NAME,
        GUIDELINES_CHANNEL_DISPLAY_NAME,
        true,
    )
}

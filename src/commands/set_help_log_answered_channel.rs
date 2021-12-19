use crate::command::templates::{SelectChannelTemplate};

pub const HELP_LOG_ANSWERED_CHANNEL_NAME: &'static str = "help_log_answered";
pub const HELP_LOG_ANSWERED_CHANNEL_DISPLAY_NAME: &'static str = "help log answered";

pub fn set_help_log_answered_channel_fn() -> SelectChannelTemplate {
    SelectChannelTemplate::new(
        HELP_LOG_ANSWERED_CHANNEL_NAME,
        HELP_LOG_ANSWERED_CHANNEL_DISPLAY_NAME,
        true,
    )
}
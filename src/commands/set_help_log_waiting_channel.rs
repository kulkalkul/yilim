use crate::command::templates::{SelectChannelTemplate};

pub const HELP_LOG_WAITING_CHANNEL_NAME: &'static str = "help_log_waiting";
pub const HELP_LOG_WAITING_CHANNEL_DISPLAY_NAME: &'static str = "help log waiting";

pub fn set_help_log_waiting_channel_fn() -> SelectChannelTemplate {
    SelectChannelTemplate::new(
        HELP_LOG_WAITING_CHANNEL_NAME,
        HELP_LOG_WAITING_CHANNEL_DISPLAY_NAME,
        true,
    )
}
use crate::command::templates::{ SetChannelTemplate };

pub const HELP_LOG_ANSWERED_CHANNEL_NAME: &'static str = "help log answered";
pub const HELP_LOG_ANSWERED_CHANNEL_ID: &'static str = "help_log_answered";

pub fn set_help_log_answered_channel_fn() -> SetChannelTemplate {
    SetChannelTemplate::new(
        HELP_LOG_ANSWERED_CHANNEL_NAME,
        HELP_LOG_ANSWERED_CHANNEL_ID,
    )
}
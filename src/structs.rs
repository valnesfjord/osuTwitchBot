use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub osu_username: String,
    pub osu_api_key: String,
    pub osu_irc_password: String,
    pub twitch_bot_username: String,
    pub twitch_bot_token: String,
    pub twitch_channel_name: String,
    pub interface_language: String,
}
pub struct App {
    pub info: String,
}

pub struct AppError {
    pub error_message: String,
}

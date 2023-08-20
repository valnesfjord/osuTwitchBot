use crate::structs::Config;
use directories::ProjectDirs;
use std::{
    fs::{self, File},
    io::Write,
};
pub fn get_config() -> Config {
    let path = ProjectDirs::from("ru", "valnesfjord", "reotbfosu").unwrap();
    if !path.config_local_dir().exists() {
        fs::create_dir_all(path.config_local_dir()).unwrap();
    }
    let mut config_path = path.config_local_dir().to_path_buf();
    config_path.push("config.toml");
    let mut config: Config = Config {
        osu_username: "".to_owned(),
        osu_api_key: "".to_owned(),
        osu_irc_password: "".to_owned(),
        twitch_bot_username: "".to_owned(),
        twitch_bot_token: "".to_owned(),
        twitch_channel_name: "".to_owned(),
        interface_language: "".to_owned(),
    };
    if config_path.exists() {
        let config_file = std::fs::read_to_string(config_path).unwrap();
        config = toml::from_str(config_file.as_str()).unwrap();
    } else {
        let mut file = File::create(config_path.as_path()).unwrap();
        file.write_all(b"osu_username=\"\"\nosu_api_key=\"\"\nosu_irc_password=\"\"\ntwitch_bot_username=\"\"\ntwitch_bot_token=\"\"\ntwitch_channel_name=\"\"\ninterface_language=\"\"\n").unwrap();
    }
    config
}

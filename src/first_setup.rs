use std::{fs::File, io::Write, rc::Rc, sync::Mutex};

use directories::ProjectDirs;

use crate::structs::Config;
pub fn first_setup(config: Config, options: eframe::NativeOptions) -> Config {
    if config.osu_username == ""
        || config.osu_api_key == ""
        || config.osu_irc_password == ""
        || config.twitch_bot_username == ""
        || config.twitch_bot_token == ""
        || config.twitch_channel_name == ""
    {
        let config_mut = Rc::new(Mutex::new(config));
        /*
        TODO: Next updates: Languages
        {
            let config_ref = Rc::clone(&config_mut);
            eframe::run_native(
                "First Setup",
                options.clone(),
                Box::new(|_cc| {
                    Box::new(FirstSetup {
                        setup_type: "language".to_owned(),
                        description: "Choose bot messages language (ru, en):".to_owned(),
                        setting_name: "Bot Language:".to_owned(),
                        setted: "".to_owned(),
                        config: config_ref,
                    })
                }),
            )
            .unwrap();
        }
         */
        {
            let config_ref = Rc::clone(&config_mut);
            eframe::run_native(
                "First Setup",
                options.clone(),
                Box::new(|_cc| {
                    Box::new(FirstSetup {
                        setup_type: "osu_username".to_owned(),
                        description: "Enter your osu! username".to_owned(),
                        setting_name: "osu! username".to_owned(),
                        setted: "".to_owned(),
                        config: config_ref,
                    })
                }),
            )
            .unwrap();
        }
        {
            let config_ref = Rc::clone(&config_mut);
            eframe::run_native(
                "First Setup",
                options.clone(),
                Box::new(|_cc| {
                    Box::new(FirstSetup {
                        setup_type: "osu_api_key".to_owned(),
                        description: "Enter your osu! API key\n(You can get it from here: https://osu.ppy.sh/p/api/ > API Key Field).\nPlease note that all data you provide here is saved on your pc locally, and not being send anywhere".to_owned(),
                        setting_name: "osu! API key".to_owned(),
                        setted: "".to_owned(),
                        config: config_ref,
                    })
                }),
            )
            .unwrap();
        }
        {
            let config_ref = Rc::clone(&config_mut);
            eframe::run_native(
                "First Setup",
                options.clone(),
                Box::new(|_cc| {
                    Box::new(FirstSetup {
                        setup_type: "osu_irc_password".to_owned(),
                        description: "Enter your IRC password for osu! chat access\n(You can get it from here: https://osu.ppy.sh/p/irc > Server Password Field).\nPlease note that all data you provide here is saved on your pc locally, and not being send anywhere".to_owned(),
                        setting_name: "osu! IRC password".to_owned(),
                        setted: "".to_owned(),
                        config: config_ref,
                    })
                }),
            )
            .unwrap();
        }
        {
            let config_ref = Rc::clone(&config_mut);
            eframe::run_native(
                "First Setup",
                options.clone(),
                Box::new(|_cc| {
                    Box::new(FirstSetup {
                        setup_type: "twitch_bot_username".to_owned(),
                        description: "Enter your twitch bot username\n(You can create new twitch account for that)".to_owned(),
                        setting_name: "Twitch bot username".to_owned(),
                        setted: "".to_owned(),
                        config: config_ref,
                    })
                }),
            )
            .unwrap();
        }
        {
            let config_ref = Rc::clone(&config_mut);
            eframe::run_native(
                "First Setup",
                options.clone(),
                Box::new(|_cc| {
                    Box::new(FirstSetup {
                        setup_type: "twitch_bot_token".to_owned(),
                        description: "Enter your twitch bot token\n(You get it from here: https://twitchapps.com/tmi/, starts with oauth)\nPlease note that all data you provide here is saved on your pc locally, and not being send anywhere".to_owned(),
                        setting_name: "Twitch bot token".to_owned(),
                        setted: "".to_owned(),
                        config: config_ref,
                    })
                }),
            )
            .unwrap();
        }
        {
            let config_ref = Rc::clone(&config_mut);
            eframe::run_native(
                "First Setup",
                options.clone(),
                Box::new(|_cc| {
                    Box::new(FirstSetup {
                        setup_type: "twitch_channel_name".to_owned(),
                        description: "Enter your twitch username (the one you broadcast from)"
                            .to_owned(),
                        setting_name: "Twitch channel name".to_owned(),
                        setted: "".to_owned(),
                        config: config_ref,
                    })
                }),
            )
            .unwrap();
        }
        {
            let conf_rc = Rc::clone(&config_mut);
            let path = ProjectDirs::from("ru", "valnesfjord", "reotbfosu").unwrap();
            let mut config_path = path.config_local_dir().to_path_buf();
            config_path.push("config.toml");
            let mut file = File::options().write(true).open(config_path).unwrap();
            let conf = conf_rc.lock().unwrap();
            file.write_all(format!("osu_username=\"{}\"\nosu_api_key=\"{}\"\nosu_irc_password=\"{}\"\ntwitch_bot_username=\"{}\"\ntwitch_bot_token=\"{}\"\ntwitch_channel_name=\"{}\"\ninterface_language=\"{}\"\n", conf.osu_username, conf.osu_api_key, conf.osu_irc_password, conf.twitch_bot_username, conf.twitch_bot_token, conf.twitch_channel_name, conf.interface_language).as_bytes()).unwrap();
        }
        Rc::clone(&config_mut).lock().unwrap().clone()
    } else {
        config
    }
}
struct FirstSetup {
    pub setup_type: String,
    pub setting_name: String,
    pub description: String,
    pub setted: String,
    pub config: Rc<Mutex<Config>>,
}
impl eframe::App for FirstSetup {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.setting_name.clone());
            if self.setup_type == "language" {
                if ui.button("EN").clicked() {
                    self.config.lock().unwrap().interface_language = "EN".to_string();
                    _frame.close();
                }
                if ui.button("RU").clicked() {
                    self.config.lock().unwrap().interface_language = "RU".to_string();
                    _frame.close();
                }
                ui.label(self.description.clone());
            } else {
                ui.horizontal(|ui| ui.text_edit_singleline(&mut self.setted));
                ui.label(self.description.clone());
                if ui.button("Submit").clicked() {
                    match self.setup_type.as_str() {
                        "osu_username" => {
                            self.config.lock().unwrap().osu_username = self.setted.to_owned()
                        }
                        "osu_api_key" => {
                            self.config.lock().unwrap().osu_api_key = self.setted.to_owned();
                        }
                        "osu_irc_password" => {
                            self.config.lock().unwrap().osu_irc_password = self.setted.to_owned();
                        }
                        "twitch_bot_username" => {
                            self.config.lock().unwrap().twitch_bot_username =
                                self.setted.to_owned();
                        }
                        "twitch_bot_token" => {
                            self.config.lock().unwrap().twitch_bot_token = self.setted.to_owned();
                        }
                        "twitch_channel_name" => {
                            self.config.lock().unwrap().twitch_channel_name =
                                self.setted.to_owned();
                        }
                        _ => (),
                    }
                    _frame.close();
                }
            }
        });
    }
}

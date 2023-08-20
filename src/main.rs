#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::{
    fs::{self, File},
    io::Write,
    process,
    rc::Rc,
    sync::Mutex,
    thread,
};
mod twitch;
use directories::ProjectDirs;
mod structs;
use image;
use std::panic;
use structs::{App, AppError, Config};
use toml;
fn load_icon() -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("C:\\Projects\\RustProjects\\reotbfosu\\assets\\icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
#[tokio::main]
async fn main() {
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
    let options = eframe::NativeOptions {
        run_and_return: true,
        initial_window_size: Some(egui::vec2(480.0, 200.0)),
        icon_data: Some(load_icon()),
        ..Default::default()
    };

    if config.osu_username == ""
        || config.osu_api_key == ""
        || config.osu_irc_password == ""
        || config.twitch_bot_username == ""
        || config.twitch_bot_token == ""
        || config.twitch_channel_name == ""
        || config.interface_language == ""
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
        let config_ref = Rc::clone(&config_mut);
        config = config_ref.lock().unwrap().clone();
    }
    let app = App {
        info: "".to_owned(),
    };
    panic::set_hook(Box::new(|i| {
        let p_info = format!("{:?}", i.payload());
        let app_options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(250.0, 400.0)),
            icon_data: Some(load_icon()),
            ..Default::default()
        };
        eframe::run_native(
            "Error!",
            app_options.clone(),
            Box::new(|_cc| {
                Box::new(AppError {
                    error_message: p_info,
                })
            }),
        )
        .unwrap();
    }));
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(twitch::twitch(config));
    });

    let app_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(200.0, 100.0)),
        icon_data: Some(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "REOTBfosu!",
        app_options.clone(),
        Box::new(|_cc| Box::new(app)),
    )
    .unwrap();
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
impl eframe::App for App {
    fn on_close_event(&mut self) -> bool {
        process::exit(0)
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("REOTBfosu!");
            ui.label("Ready");
            if ui.button("Delete configs").clicked() {
                let path = ProjectDirs::from("ru", "valnesfjord", "reotbfosu").unwrap();
                let mut config_path = path.config_local_dir().to_path_buf();
                config_path.push("config.toml");
                fs::remove_file(config_path).unwrap();
                process::exit(-1);
            }
        });
    }
}
impl eframe::App for AppError {
    fn on_close_event(&mut self) -> bool {
        process::exit(-1);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("An error occurred!");
            ui.label(self.error_message.clone());
            if ui.button("Delete configs").clicked() {
                let path = ProjectDirs::from("ru", "valnesfjord", "reotbfosu").unwrap();
                let mut config_path = path.config_local_dir().to_path_buf();
                config_path.push("config.toml");
                fs::remove_file(config_path).unwrap();
                process::exit(-1);
            }
        });
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::{
    fs::{self},
    process, thread,
};
mod twitch;
use directories::ProjectDirs;
mod structs;
use image;
use std::panic;
use structs::{App, AppError};
mod config;
mod first_setup;
use config::get_config;
use first_setup::first_setup;
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
    let mut config = get_config();
    let options = eframe::NativeOptions {
        run_and_return: true,
        initial_window_size: Some(egui::vec2(480.0, 200.0)),
        icon_data: Some(load_icon()),
        ..Default::default()
    };
    config = first_setup(config, options.clone());
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

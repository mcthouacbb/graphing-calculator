use crate::app::settings::Settings;
use eframe::egui;

mod settings;

pub struct App {
    settings: Settings,
    show_settings: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            show_settings: false,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::top("menu").show_inside(ui, |ui| {
            if ui.button("Settings").clicked() {
                self.show_settings = true;
            }
        });

        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .show(ui.ctx(), |ui| {});
        }
        egui::Panel::left("equations").show_inside(ui, |ui| {
            // todo
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // todo
        });
    }
}

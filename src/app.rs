use crate::app::{camera::Camera, settings::Settings};
use eframe::egui;

mod camera;
mod settings;
mod widgets;

pub struct App {
    settings: Settings,
    show_settings: bool,

    camera: Camera,
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            show_settings: false,
            camera: Camera::home(1.0),
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
                .show(ui.ctx(), |ui| self.settings.ui(ui, &mut self.camera));
        }
        egui::Panel::left("equations").show_inside(ui, |ui| {
            // todo
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // todo
        });
    }
}

use crate::app::{camera::Camera, equation_editor::EquationEditor, settings::Settings};
use eframe::egui;

mod camera;
mod equation_editor;
mod settings;
mod widgets;

pub struct App {
    settings: Settings,
    show_settings: bool,

    camera: Camera,
    equations: Vec<EquationEditor>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            show_settings: false,
            camera: Camera::home(1.0),
            equations: vec![EquationEditor::new()],
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
        egui::Panel::left("equation_editors").show_inside(ui, |ui| {
            for (idx, equation) in self.equations.iter_mut().enumerate() {
                ui.push_id(idx, |ui| {
                    equation.ui(ui);
                });
            }
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // todo
        });
    }
}

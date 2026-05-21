use crate::app::{camera::Camera, widgets::float_input};

use eframe::egui;

#[derive(Default)]
pub struct Settings {
    camera_left: String,
    camera_right: String,
    camera_bottom: String,
    camera_top: String,
}

impl Settings {
    pub fn ui(&mut self, ui: &mut egui::Ui, camera: &mut Camera) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Camera Bounds X");
            ui.columns(2, |cols| {
                cols[0].label("Left: ");
                float_input(&mut cols[0], &mut self.camera_left, &mut camera.left);
                cols[1].label("Right: ");
                float_input(&mut cols[1], &mut self.camera_right, &mut camera.right);
            });
            ui.label("Camera Bounds Y");
            ui.columns(2, |cols| {
                cols[0].label("Bottom: ");
                float_input(&mut cols[0], &mut self.camera_bottom, &mut camera.bottom);
                cols[1].label("Top: ");
                float_input(&mut cols[1], &mut self.camera_top, &mut camera.top);
            });
        });
    }
}

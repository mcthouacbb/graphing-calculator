use crate::app::{camera::Camera, widgets::float_input};

use eframe::egui;

struct AxisSettings {
    render: bool,
    render_ticks: bool,
    render_grid: bool,
}

impl Default for AxisSettings {
    fn default() -> Self {
        Self {
            render: true,
            render_ticks: true,
            render_grid: true,
        }
    }
}

#[derive(Default)]
pub struct Settings {
    camera_left: String,
    camera_right: String,
    camera_bottom: String,
    camera_top: String,

    x_axis: AxisSettings,
    y_axis: AxisSettings,
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

            Self::axis_settings(ui, &mut self.x_axis, "x");
            Self::axis_settings(ui, &mut self.y_axis, "y");
        });
    }

    fn axis_settings(ui: &mut egui::Ui, axis: &mut AxisSettings, name: &str) {
        ui.label(format!("{} axis", name));
        ui.checkbox(&mut axis.render, "Show axis");
        ui.checkbox(&mut axis.render_ticks, "Show labels");
        ui.checkbox(&mut axis.render_grid, "Show grid lines");
    }
}

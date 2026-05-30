use crate::app::{camera::Camera, widgets::float_input};

use eframe::egui;

pub struct AxisSettings {
    render_axis: bool,
    render_ticks: bool,
}

impl AxisSettings {
    pub fn render_axis(&self) -> bool {
        self.render_axis
    }

    pub fn render_ticks(&self) -> bool {
        self.render_ticks
    }
}

impl Default for AxisSettings {
    fn default() -> Self {
        Self {
            render_axis: true,
            render_ticks: true,
        }
    }
}

pub struct Settings {
    camera_left: String,
    camera_right: String,
    camera_bottom: String,
    camera_top: String,

    x_axis: AxisSettings,
    y_axis: AxisSettings,
    render_grid: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera_left: String::new(),
            camera_right: String::new(),
            camera_bottom: String::new(),
            camera_top: String::new(),

            x_axis: AxisSettings::default(),
            y_axis: AxisSettings::default(),
            render_grid: true,
        }
    }
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

            ui.checkbox(&mut self.render_grid, "Show grid lines");
            Self::axis_settings(ui, &mut self.x_axis, "x");
            Self::axis_settings(ui, &mut self.y_axis, "y");
        });
    }

    pub fn x_axis(&self) -> &AxisSettings {
        &self.x_axis
    }

    pub fn y_axis(&self) -> &AxisSettings {
        &self.y_axis
    }

    pub fn render_grid(&self) -> bool {
        self.render_grid
    }

    fn axis_settings(ui: &mut egui::Ui, axis: &mut AxisSettings, name: &str) {
        ui.label(format!("{} axis", name));
        ui.checkbox(&mut axis.render_axis, "Show axis");
        ui.checkbox(&mut axis.render_ticks, "Show labels");
    }
}

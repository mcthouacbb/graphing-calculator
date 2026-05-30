use crate::{
    app::{camera::Camera, equation_editor::EquationEditor, settings::Settings},
    renderer,
};

use eframe::egui;

pub mod camera;
mod equation_editor;
pub mod settings;
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

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show_inside(ui, |ui| {
                let response =
                    ui.interact(ui.max_rect(), ui.id().with("drag"), egui::Sense::drag());

                if response.dragged_by(egui::PointerButton::Primary) {
                    let delta = response.drag_delta() / ui.max_rect().size();
                    self.camera.translate(delta.x as f64, delta.y as f64);
                }

                ui.input(|input| {
                    if let Some(pointer_pos) = input.pointer.latest_pos() {
                        let pos = (pointer_pos - ui.max_rect().min) / ui.max_rect().size();
                        self.camera.zoom(
                            pos.x as f64,
                            pos.y as f64,
                            input.smooth_scroll_delta().y as f64,
                        );
                    }
                });

                let width = ui.max_rect().width().ceil() as usize;
                let height = ui.max_rect().height().ceil() as usize;

                if width > 0 && height > 0 {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 0.0, egui::Color32::WHITE);
                    renderer::render_graph_structure(
                        &self.camera,
                        &self.settings,
                        ui,
                        width,
                        height,
                    );

                    let mut equations = Vec::with_capacity(self.equations.len());
                    for equation_editor in &self.equations {
                        if let Some(equation) = equation_editor.equation() {
                            equations.push(equation);
                        }
                    }
                    renderer::render(&self.camera, width, height, ui, &equations);
                }

                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::right_to_left(egui::Align::Min),
                    |ui| {
                        if ui.button("Home").clicked() {
                            self.camera = Camera::home(width as f64 / height as f64);
                        }
                    },
                );
            });
    }
}

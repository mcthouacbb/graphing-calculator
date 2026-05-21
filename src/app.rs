use crate::{
    app::{camera::Camera, equation_editor::EquationEditor, settings::Settings},
    renderer,
};

use eframe::egui;

pub mod camera;
mod equation_editor;
mod settings;
mod widgets;

pub struct App {
    settings: Settings,
    show_settings: bool,

    camera: Camera,
    equations: Vec<EquationEditor>,

    texture: Option<egui::TextureHandle>,
    framebuffer: Vec<egui::Color32>,
    fb_size: [usize; 2],
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            show_settings: false,
            camera: Camera::home(1.0),
            equations: vec![EquationEditor::new()],

            texture: None,
            framebuffer: Vec::new(),
            fb_size: [0; 2],
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
                let width = ui.max_rect().width().ceil() as usize;
                let height = ui.max_rect().height().ceil() as usize;

                if self.fb_size[0] != width || self.fb_size[1] != height {
                    self.framebuffer
                        .resize(width * height, egui::Color32::BLACK);
                    self.fb_size = [width, height]
                }

                if width > 0 && height > 0 {
                    renderer::render(&self.camera, width, height, &mut self.framebuffer);
                    let image = egui::ColorImage::new(self.fb_size, self.framebuffer.clone());

                    let texture = if let Some(texture) = self.texture.as_mut() {
                        texture.set(image, egui::TextureOptions::LINEAR);
                        texture
                    } else {
                        self.texture = Some(ui.ctx().load_texture(
                            "framebuffer",
                            image,
                            egui::TextureOptions::LINEAR,
                        ));
                        self.texture.as_ref().unwrap()
                    };
                    ui.painter().image(
                        texture.id(),
                        egui::Rect::from_min_size(ui.max_rect().min, texture.size_vec2()),
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
            });
    }
}

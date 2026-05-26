use crate::{app::camera::Camera, equation::explicit::ExplicitEquation};

use eframe::egui;

pub fn graph_explicit_equation(
    camera: &Camera,
    width: usize,
    height: usize,
    ui: &mut egui::Ui,
    equation: &ExplicitEquation,
) {
    let mut prev = None;

    for x in 0..width {
        let wx = camera.screen_to_world_x((x as f64 + 0.5) / width as f64);
        let wy = equation.calc(wx);

        if let Some((prev_wx, prev_wy)) = prev {
            let (cx, cy) = camera.world_to_screen(wx, wy);
            let (prev_cx, prev_cy) = camera.world_to_screen(prev_wx, prev_wy);

            ui.painter().line_segment(
                [
                    ui.max_rect().min
                        + egui::vec2(
                            width as f32 * prev_cx as f32,
                            height as f32 * prev_cy as f32,
                        ),
                    ui.max_rect().min
                        + egui::vec2(width as f32 * cx as f32, height as f32 * cy as f32),
                ],
                egui::Stroke::new(2.0, egui::Color32::BLACK),
            );
        }

        prev = Some((wx, wy))
    }
}

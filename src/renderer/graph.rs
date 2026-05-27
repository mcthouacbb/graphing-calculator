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

    for x in (0..width).step_by(8) {
        let wx = camera.screen_to_world_x((x as f64 + 0.5) / width as f64);
        let wy = equation.calc(wx);

        if let Some((prev_wx, prev_wy)) = prev {
            render_segment(
                camera, width, height, ui, equation, prev_wx, prev_wy, wx, wy,
            );
        }

        prev = Some((wx, wy))
    }
}

pub fn render_segment(
    camera: &Camera,
    width: usize,
    height: usize,
    ui: &mut egui::Ui,
    equation: &ExplicitEquation,
    prev_wx: f64,
    prev_wy: f64,
    wx: f64,
    wy: f64,
) {
    let cx_diff =
        (camera.world_to_screen_x(wx) - camera.world_to_screen_x(prev_wx)).abs() * width as f64;
    let cy_diff =
        (camera.world_to_screen_y(wy) - camera.world_to_screen_y(prev_wy)).abs() * height as f64;

    let subdivide = if cx_diff < 0.01 {
        false
    } else if !prev_wy.is_finite() || !wy.is_finite() || cy_diff > 8.0 {
        true
    } else {
        // TODO: use interval arithmetic to estimate aliasing/discontinuity
        let mid_wx = (prev_wx + wx) / 2.0;
        let actual_mid_wy = equation.calc(mid_wx);
        let expected_mid_wy = (prev_wy + wy) / 2.0;

        let mid_cy_diff = (camera.world_to_screen_y(actual_mid_wy)
            - camera.world_to_screen_y(expected_mid_wy))
        .abs()
            * height as f64;

        mid_cy_diff >= 0.25 || !mid_cy_diff.is_finite()
    };

    if subdivide {
        let mid_wx = (prev_wx + wx) / 2.0;
        let mid_wy = equation.calc(mid_wx);
        if prev_wy.is_finite() || mid_wx.is_finite() {
            render_segment(
                camera, width, height, ui, equation, prev_wx, prev_wy, mid_wx, mid_wy,
            );
        }
        if mid_wy.is_finite() || wy.is_finite() {
            render_segment(camera, width, height, ui, equation, mid_wx, mid_wy, wx, wy);
        }
    } else {
        let (cx, cy) = camera.world_to_screen(wx, wy);
        let (prev_cx, prev_cy) = camera.world_to_screen(prev_wx, prev_wy);

        ui.painter().line_segment(
            [
                ui.max_rect().min
                    + egui::vec2(
                        width as f32 * prev_cx as f32,
                        height as f32 * prev_cy as f32,
                    ),
                ui.max_rect().min + egui::vec2(width as f32 * cx as f32, height as f32 * cy as f32),
            ],
            egui::Stroke::new(2.0, egui::Color32::RED),
        );
    }
}

use crate::{
    app::{camera::Camera, settings::Settings},
    equation::explicit::ExplicitEquation,
    interval::Interval,
};

use eframe::egui;

pub fn graph_explicit_equation(
    camera: &Camera,
    settings: &Settings,
    width: usize,
    height: usize,
    ui: &mut egui::Ui,
    equation: &ExplicitEquation,
) {
    let mut prev = None;

    for x in (0..=width).step_by(8) {
        let wx = camera.screen_to_world_x((x as f64 + 0.5) / width as f64);
        let wy = equation.calc(wx);

        if let Some((prev_wx, prev_wy)) = prev {
            let y_interval = equation.calc_interval(&Interval::new(prev_wx, wx));
            render_segment(
                camera, settings, width, height, ui, equation, prev_wx, prev_wy, wx, wy, y_interval,
            );
        }

        prev = Some((wx, wy))
    }
}

pub fn render_segment(
    camera: &Camera,
    settings: &Settings,
    width: usize,
    height: usize,
    ui: &mut egui::Ui,
    equation: &ExplicitEquation,
    prev_wx: f64,
    prev_wy: f64,
    wx: f64,
    wy: f64,
    y_interval: Interval,
) {
    let cx_diff =
        (camera.world_to_screen_x(wx) - camera.world_to_screen_x(prev_wx)).abs() * width as f64;
    let cy_diff =
        (camera.world_to_screen_y(wy) - camera.world_to_screen_y(prev_wy)).abs() * height as f64;

    if y_interval.empty() || y_interval.lower() > camera.top || y_interval.upper() < camera.bottom {
        return;
    }

    let mid_wx = (prev_wx + wx) / 2.0;
    let mid_wy = equation.calc(mid_wx);
    let expected_mid_wy = (prev_wy + wy) / 2.0;

    let left_interval = equation.calc_interval(&Interval::new(prev_wx, mid_wx));
    let right_interval = equation.calc_interval(&Interval::new(mid_wx, wx));

    let mid_cy_diff =
        (camera.world_to_screen_y(mid_wy) - camera.world_to_screen_y(expected_mid_wy)).abs()
            * height as f64;

    let likely_alias = right_interval.length() > y_interval.length() / 2.0
        && left_interval.length() > y_interval.length() / 2.0
        && ((wy - prev_wy).abs() < y_interval.length() / 2.0 || cx_diff >= 1.0);

    let subdivide = if cx_diff < 0.1 {
        false
    } else if !prev_wy.is_finite() || !wy.is_finite() || cy_diff > 8.0 {
        true
    } else {
        mid_cy_diff >= 0.25
            || !mid_cy_diff.is_finite()
            || !y_interval.is_finite()
            || !y_interval.continuous()
            || likely_alias
    };

    if subdivide {
        if prev_wy.is_finite() || mid_wx.is_finite() {
            render_segment(
                camera,
                settings,
                width,
                height,
                ui,
                equation,
                prev_wx,
                prev_wy,
                mid_wx,
                mid_wy,
                left_interval,
            );
        }
        if mid_wy.is_finite() || wy.is_finite() {
            render_segment(
                camera,
                settings,
                width,
                height,
                ui,
                equation,
                mid_wx,
                mid_wy,
                wx,
                wy,
                right_interval,
            );
        }
    } else if wy.is_finite()
        && prev_wy.is_finite()
        && y_interval.is_finite()
        && y_interval.continuous()
    {
        let (cx, cy) = camera.world_to_screen(wx, wy);
        let (prev_cx, prev_cy) = camera.world_to_screen(prev_wx, prev_wy);

        if cy > 1.0 && prev_cy > 1.0 || cy < 0.0 && prev_cy < 0.0 {
            return;
        }

        if settings.show_debug_subdivisions() {
            ui.painter().vline(
                ui.max_rect().min.x + width as f32 * prev_cx as f32,
                ui.max_rect().y_range(),
                egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(0, 0, 139, 128)),
            );

            ui.painter().vline(
                ui.max_rect().min.x + width as f32 * cx as f32,
                ui.max_rect().y_range(),
                egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(0, 0, 139, 128)),
            );
        }

        if settings.show_debug_intervals() {
            ui.painter().rect_filled(
                egui::Rect::from_points(&[
                    ui.max_rect().min
                        + egui::vec2(
                            width as f32 * prev_cx as f32,
                            height as f32 * camera.world_to_screen_y(y_interval.lower()) as f32,
                        ),
                    ui.max_rect().min
                        + egui::vec2(
                            width as f32 * cx as f32,
                            height as f32 * camera.world_to_screen_y(y_interval.upper()) as f32,
                        ),
                ]),
                0,
                egui::Color32::GREEN,
            );
        }

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

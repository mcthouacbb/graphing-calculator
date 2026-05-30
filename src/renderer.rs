use crate::{
    app::{camera::Camera, settings::Settings},
    equation::Equation,
    renderer::{
        graph::graph_explicit_equation,
        structure::{fmt_tick_pos, tick_width},
    },
};

use eframe::egui;

mod graph;
mod structure;

pub fn render(
    camera: &Camera,
    width: usize,
    height: usize,
    ui: &mut egui::Ui,
    equations: &[&Equation],
) {
    for equation in equations {
        match equation {
            Equation::Explicit(explicit) => {
                graph_explicit_equation(camera, width, height, ui, explicit);
            }
        }
    }
}

pub fn render_graph_structure(
    camera: &Camera,
    settings: &Settings,
    ui: &egui::Ui,
    width: usize,
    height: usize,
) {
    let (origin_x, origin_y) = camera.world_to_screen(0.0, 0.0);

    let min_x = ui.max_rect().min.x;
    let min_y = ui.max_rect().min.y;

    let tick_width_x = tick_width((camera.right - camera.left) * height as f64 / width as f64);
    let tick_width_y = tick_width(camera.top - camera.bottom);

    let mut tick_x = tick_width_x * (camera.left / tick_width_x).ceil();
    while tick_x < camera.right {
        let (tick_screen_x, _) = camera.world_to_screen(tick_x, 0.0);

        if settings.render_grid() {
            ui.painter().vline(
                min_x + (tick_screen_x * width as f64) as f32,
                ui.max_rect().y_range(),
                egui::Stroke::new(1.5, egui::Color32::GRAY),
            );
        }

        if settings.x_axis().render_ticks() && (tick_screen_x - origin_x).abs() > 0.01 {
            let text_layout = ui.fonts_mut(|fonts| {
                fonts.layout_no_wrap(
                    fmt_tick_pos(tick_x),
                    egui::FontId::proportional(12.0),
                    egui::Color32::BLACK,
                )
            });
            let target_pos = egui::pos2(
                min_x + (tick_screen_x * width as f64) as f32 - text_layout.size().x / 2.0,
                min_y + (origin_y * height as f64) as f32 - text_layout.size().y,
            );

            let pos = egui::pos2(
                target_pos.x.clamp(
                    ui.max_rect().left(),
                    ui.max_rect().right() - text_layout.size().x,
                ),
                target_pos.y.clamp(
                    ui.max_rect().top(),
                    ui.max_rect().bottom() - text_layout.size().y,
                ),
            );

            ui.painter().rect_filled(
                text_layout.rect.translate(pos.to_vec2()),
                0.0,
                egui::Color32::WHITE,
            );
            ui.painter().galley(pos, text_layout, egui::Color32::WHITE);
        }

        tick_x += tick_width_x;
    }

    let mut tick_y = tick_width_y * (camera.bottom / tick_width_y).ceil();
    while tick_y < camera.top {
        let (_, tick_screen_y) = camera.world_to_screen(0.0, tick_y);

        if settings.render_grid() {
            ui.painter().hline(
                ui.max_rect().x_range(),
                min_y + (tick_screen_y * height as f64) as f32,
                egui::Stroke::new(1.5, egui::Color32::GRAY),
            );
        }

        if settings.y_axis().render_ticks() && (tick_screen_y - origin_y).abs() > 0.01 {
            let text_layout = ui.fonts_mut(|fonts| {
                fonts.layout_no_wrap(
                    fmt_tick_pos(tick_y),
                    egui::FontId::proportional(12.0),
                    egui::Color32::BLACK,
                )
            });
            let target_pos = egui::pos2(
                min_x + (origin_x * width as f64) as f32 + 5.0,
                min_y + (tick_screen_y * height as f64) as f32 - text_layout.size().y,
            );

            let pos = egui::pos2(
                target_pos.x.clamp(
                    ui.max_rect().left(),
                    ui.max_rect().right() - text_layout.size().x,
                ),
                target_pos.y.clamp(
                    ui.max_rect().top(),
                    ui.max_rect().bottom() - text_layout.size().y,
                ),
            );

            ui.painter().galley(pos, text_layout, egui::Color32::WHITE);
        }

        tick_y += tick_width_y;
    }

    // x-axis
    if origin_y >= 0.0 && origin_y <= 1.0 && settings.x_axis().render_axis() {
        ui.painter().hline(
            ui.max_rect().x_range(),
            min_y + (origin_y * height as f64) as f32,
            egui::Stroke::new(2.5, egui::Color32::BLACK),
        );
    }

    if origin_x >= 0.0 && origin_x <= 1.0 && settings.y_axis().render_axis() {
        ui.painter().vline(
            min_x + (origin_x * width as f64) as f32,
            ui.max_rect().y_range(),
            egui::Stroke::new(2.5, egui::Color32::BLACK),
        );
    }
}

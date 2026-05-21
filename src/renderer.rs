use crate::{
    app::{camera::Camera, settings::Settings},
    renderer::graph::graph_equation,
};

use eframe::egui;

mod graph;

pub fn render(camera: &Camera, width: usize, height: usize, framebuffer: &mut Vec<egui::Color32>) {
    for y in 0..height {
        for x in 0..width {
            let r = (0.5 + 255.0 * (y as f32 / height as f32)) as u8;
            let g = (0.5 + 255.0 * (x as f32 / width as f32)) as u8;
            let b = 127u8;
            framebuffer[y * width + x] = egui::Color32::from_rgb(r, g, b);
        }
    }
    graph_equation(camera, width, height, framebuffer, |x, y| x.powi(2) - y);
    graph_equation(camera, width, height, framebuffer, |x, y| {
        x.powi(2) + y.powi(2) - 1.0
    });
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
    // x-axis
    if origin_y >= 0.0 && origin_y <= 1.0 {
        ui.painter().hline(
            ui.max_rect().x_range(),
            min_y + (origin_y * height as f64 - 0.5) as f32,
            egui::Stroke::new(2.5, egui::Color32::BLACK),
        );
    }

    if origin_x >= 0.0 && origin_x <= 1.0 {
        ui.painter().vline(
            min_x + (origin_x * width as f64 - 0.5) as f32,
            ui.max_rect().y_range(),
            egui::Stroke::new(2.5, egui::Color32::BLACK),
        );
    }
}

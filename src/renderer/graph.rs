use crate::app::camera::Camera;

use eframe::egui;

pub fn graph_equation<E: Fn(f64, f64) -> f64>(
    camera: &Camera,
    width: usize,
    height: usize,
    framebuffer: &mut Vec<egui::Color32>,
    equation: E,
) {
    for y in 0..height {
        for x in 0..width {
            let mut nonnegative = false;
            let mut nonpositive = false;
            for corner in 0..4 {
                let cx = (x + corner % 2) as f64 / width as f64;
                let cy = (y + corner / 2) as f64 / height as f64;
                let (wx, wy) = camera.screen_to_world(cx, cy);
                let sign = equation(wx, wy);
                if sign >= 0.0 {
                    nonnegative = true;
                }
                if sign <= 0.0 {
                    nonpositive = true;
                }
            }
            if nonnegative && nonpositive {
                framebuffer[y * width + x] = egui::Color32::BLACK;
            }
        }
    }
}

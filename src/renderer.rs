use eframe::egui;

pub fn render(width: usize, height: usize, framebuffer: &mut Vec<egui::Color32>) {
    for y in 0..height {
        for x in 0..width {
            let r = (0.5 + 255.0 * (y as f32 / height as f32)) as u8;
            let g = (0.5 + 255.0 * (x as f32 / width as f32)) as u8;
            let b = 127u8;
            framebuffer[y * width + x] = egui::Color32::from_rgb(r, g, b);
        }
    }
}

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let result = eframe::run_native(
        "Graphing Calculator",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyEguiApp::default()))),
    );

    if let Err(err) = result {
        eprintln!("Failed to start graphing calculator: {}", err);
    }
}

#[derive(Default)]
struct MyEguiApp {
    label: String,
    value: f32,
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Hello World!");
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        });
    }
}

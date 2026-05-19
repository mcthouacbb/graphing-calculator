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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorChoice {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppMode {
    View,
    Edit,
    Settings,
}

struct MyEguiApp {
    label: String,
    value: f32,
    show_extra_info: bool,
    selected_color: ColorChoice,
    counter: i32,
    current_mode: AppMode,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            label: "Initial Text".to_string(),
            value: 5.0,
            show_extra_info: false,
            selected_color: ColorChoice::Red,
            counter: 0,
            current_mode: AppMode::View,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("mode_switcher").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Mode: ");
                ui.radio_value(&mut self.current_mode, AppMode::View, "View");
                ui.radio_value(&mut self.current_mode, AppMode::Edit, "Edit");
                ui.radio_value(&mut self.current_mode, AppMode::Settings, "Settings");
            })
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading(format!("Current mode: {:?}", self.current_mode));
            ui.separator();

            match self.current_mode {
                AppMode::View => {
                    ui.label("Viewing Data (Read Only):");
                    ui.label(format!("Label: {}", self.label));
                    ui.label(format!("Value: {:.1}", self.value));
                    if self.show_extra_info {
                        ui.label(format!("Counter: {:?}", self.counter));
                        ui.label(format!("Color: {:?}", self.selected_color));
                    } else {
                        ui.label("(Enable 'Show Advanced Info' in Settings to see more)");
                    }
                    if ui.button("Switch to edit mode").clicked() {
                        self.current_mode = AppMode::Edit;
                    }
                }
                AppMode::Edit => {
                    ui.label("Editing data:");
                    egui::Grid::new("edit_grid")
                        .num_columns(2)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Label: ");
                            ui.text_edit_singleline(&mut self.label);
                            ui.end_row();

                            ui.label("Value: ");
                            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0));
                            ui.end_row();

                            ui.label("Counter:");
                            ui.horizontal(|ui| {
                                if ui.button("+").clicked() {
                                    self.counter += 1;
                                }
                                ui.label(format!("{}", self.counter));
                                if ui.button("-").clicked() {
                                    self.counter -= 1;
                                }
                            });
                            ui.end_row();
                        });
                }
                AppMode::Settings => {
                    ui.label("Application settings:");
                    ui.separator();

                    ui.checkbox(&mut self.show_extra_info, "Show Advanced Info");
                    ui.separator();

                    ui.label("Color Scheme");
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.selected_color, ColorChoice::Red, "Red");
                        ui.radio_value(&mut self.selected_color, ColorChoice::Green, "Green");
                        ui.radio_value(&mut self.selected_color, ColorChoice::Blue, "Blue");
                    });
                    ui.label(format!("Selected color: {:?}", self.selected_color));

                    ui.separator();

                    if ui.button("Reset All State").clicked() {
                        *self = MyEguiApp::default();
                        self.current_mode = AppMode::Settings;
                    }
                }
            }
        });
    }
}

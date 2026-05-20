use eframe::egui;

pub struct EquationEditor {
    data: String,
}

impl EquationEditor {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.data);
    }

    pub fn data(&self) -> &str {
        self.data.as_str()
    }
}

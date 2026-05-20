use eframe::egui;

pub fn float_input(ui: &mut egui::Ui, s: &mut String, value: &mut f64) {
    let focused = ui.text_edit_singleline(s).has_focus();
    let result = s.parse::<f64>().ok();
    if let Some(new_value) = result {
        *value = new_value;
    }
    if !focused {
        *s = value.to_string();
    }
}

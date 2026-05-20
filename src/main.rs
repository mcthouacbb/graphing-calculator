use crate::app::App;

mod app;
mod renderer;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let result = eframe::run_native(
        "Graphing Calculator",
        native_options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    );

    if let Err(err) = result {
        eprintln!("Failed to start graphing calculator: {}", err);
    }
}

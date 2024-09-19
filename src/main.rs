mod gui;

use gui::app::ReplayApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _app = eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(ReplayApp::new(cc)))));
}

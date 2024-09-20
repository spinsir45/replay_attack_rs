use crate::gui::app::ReplayApp;

impl ReplayApp {
    pub fn signals_tab(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("signals tab");
        });
    }
}

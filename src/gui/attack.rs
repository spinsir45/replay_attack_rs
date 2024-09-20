use crate::gui::app::ReplayApp;

impl ReplayApp {
    pub fn attack_tab(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("attack tab");
        });
    }
}

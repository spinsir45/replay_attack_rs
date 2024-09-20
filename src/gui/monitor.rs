use crate::gui::app::ReplayApp;

impl ReplayApp {
    pub fn monitor_tab(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("sdr_settings").show_inside(ui, |ui| {
            ui.label("sdr settings");
        });

        egui::CentralPanel::default().show_inside(ui, |ui| ui.label("monitor tab"));

        egui::SidePanel::right("monitor_plot")
            .default_width(150.0)
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.label("side area");
            });
    }
}

use crate::gui::app::ReplayApp;
use crate::gui::chart::signals_chart;

impl ReplayApp {
    pub fn signals_tab(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::SidePanel::left("monitor_settings")
            .min_width(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.label("signals tab");
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            signals_chart(ui);
    
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                let _save_button = ui.button("Save");
                let _start_stop = ui.button("Start");
            });
        });
    }
}

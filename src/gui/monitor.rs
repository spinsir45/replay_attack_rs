use crate::gui::app::ReplayApp;

impl ReplayApp {
    pub fn monitor_tab(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("sdr_settings")
        .max_height(70.0)
        .show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.label("Center Freq (mHz):");
                    ui.add_sized(egui::Vec2::new(60.0, 10.0), egui::TextEdit::singleline(&mut self.center_frequency));
                });
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.label("Sample Rate (mHz):");
                    ui.add_sized(egui::Vec2::new(60.0, 10.0), egui::TextEdit::singleline(&mut self.sample_rate));
                });
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.label("TX Gain:");
                    ui.add_sized(egui::Vec2::new(45.0, 10.0), egui::TextEdit::singleline(&mut self.tx_gain));
                });
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.label("RX Gain:");
                    ui.add_sized(egui::Vec2::new(45.0, 10.0), egui::TextEdit::singleline(&mut self.rx_gain));
                });
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.add_space(10.0);
                    let _apply_button = ui.button("Apply");
                });
            });
        });

        egui::SidePanel::left("monitor_plot")
            .min_width(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.label("side area");
            });
        
        egui::CentralPanel::default().show_inside(ui, |ui| ui.label("monitor tab"));
    }
}

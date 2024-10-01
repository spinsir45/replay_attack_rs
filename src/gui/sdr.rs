use crate::gui::app::ReplayApp;

pub fn sdr_ui(app: &mut ReplayApp,ui: &mut egui::Ui) {
    ui.heading("SDR Settings");
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.label("Center Freq (mHz):");
            ui.add_sized(egui::Vec2::new(60.0, 10.0), egui::TextEdit::singleline(&mut app.center_frequency));
        });
        ui.add_space(10.0);

        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.label("Sample Rate (mHz):");
            ui.add_sized(egui::Vec2::new(60.0, 10.0), egui::TextEdit::singleline(&mut app.sample_rate));
        });
        ui.add_space(10.0);

        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.label("TX Gain:");
            ui.add_sized(egui::Vec2::new(45.0, 10.0), egui::TextEdit::singleline(&mut app.tx_gain));
        });
        ui.add_space(10.0);

        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.label("RX Gain:");
            ui.add_sized(egui::Vec2::new(45.0, 10.0), egui::TextEdit::singleline(&mut app.rx_gain));
        });
        ui.add_space(10.0);

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.add_space(3.0);
            let _apply_button = ui.button("Apply");
        });
    });
    ui.add_space(3.0);
}

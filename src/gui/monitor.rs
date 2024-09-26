use crate::gui::app::ReplayApp;

#[derive(Debug, PartialEq)]
pub enum MonitorMode {
    Manual,
    Auto
}

impl ReplayApp {
    pub fn monitor_tab(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("sdr_settings")
        .max_height(65.0)
        .show_inside(ui, |ui| {
            ui.heading("SDR Settings");
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
                    ui.label("RX Gain:");
                    ui.add_sized(egui::Vec2::new(45.0, 10.0), egui::TextEdit::singleline(&mut self.rx_gain));
                });
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.add_space(3.0);
                    let _apply_button = ui.button("Apply");
                });
            });
            ui.add_space(3.0);
        });

        egui::SidePanel::left("monitor_plot")
            .min_width(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.heading("Monitor Settings");

                egui::ComboBox::from_label("Select Mode")
                    .selected_text(format!("{:?}", self.monitor_mode))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.monitor_mode, MonitorMode::Manual, "Manual");
                        ui.selectable_value(&mut self.monitor_mode, MonitorMode::Auto, "Auto Detect");
                    });
            });
        
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                let sin: egui_plot::PlotPoints = (0..1000).map(|i| {
                    let x = i as f64 * 0.01;
                    [x, x.sin()]
                }).collect();
                let line = egui_plot::Line::new(sin);
                egui_plot::Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                let _start_stop = ui.button("Start");
                let _save_button = ui.button("Save");
            });
        });
    }
}

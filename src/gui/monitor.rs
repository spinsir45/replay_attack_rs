use crate::gui::app::ReplayApp;

#[derive(Debug, PartialEq)]
pub enum MonitorMode {
    Manual,
    Auto
}

impl MonitorMode {
    pub fn is_manual(&self) -> bool {
        if self.eq(&MonitorMode::Manual) {
            true
        } else {
            false
        }
    }
}

#[derive(PartialEq)]
pub struct MonitorSettings {
    mode: MonitorMode,
    time: f32,
    threshold: f32,
    cutoff: f32,
    slack: f32,
}

impl Default for MonitorSettings {
    fn default() -> Self {
        MonitorSettings {
            mode: MonitorMode::Manual,
            time: 2.5,
            threshold: 5.0,
            cutoff: 500.0,
            slack: 100.0,
        }
    }
}

impl ReplayApp {
    pub fn monitor_tab(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("monitor_sdr_settings")
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

        egui::SidePanel::left("monitor_settings")
            .min_width(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.heading("Monitor Settings");
                ui.add_space(10.0);

                ui.label("Mode:");
                egui::ComboBox::from_id_source("mode")
                    .selected_text(format!("{:?}", self.monitor_settings.mode))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.monitor_settings.mode, MonitorMode::Manual, "Manual");
                        ui.selectable_value(&mut self.monitor_settings.mode, MonitorMode::Auto, "Auto Detect");
                    });

                ui.label("Time Frame:");
                let time_widget = egui::DragValue::new(&mut self.monitor_settings.time).range(0.2..=5.0).speed(0.01).max_decimals(1);
                ui.add(time_widget);

                if self.monitor_settings.mode.is_manual() {
                    self.enable_auto_settings = false;
                } else {
                    self.enable_auto_settings = true;
                }
                
                ui.add_enabled_ui(self.enable_auto_settings, |ui| {
                    ui.label("Threshold:");
                    let threshold_value = egui::DragValue::new(&mut self.monitor_settings.threshold).range(0..=2000).max_decimals(1);
                    ui.add(threshold_value);

                    ui.label("Cutoff:");
                    let threshold_value = egui::DragValue::new(&mut self.monitor_settings.cutoff).range(100..=10000).max_decimals(1);
                    ui.add(threshold_value);

                    ui.label("Slack:");
                    let threshold_value = egui::DragValue::new(&mut self.monitor_settings.slack).range(100..=5000).max_decimals(1);
                    ui.add(threshold_value);
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

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                let _save_button = ui.button("Save");
                let _start_stop = ui.button("Start");
            });
        });
    }
}

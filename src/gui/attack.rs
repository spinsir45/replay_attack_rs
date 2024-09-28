use crate::gui::app::ReplayApp;

#[derive(PartialEq)]
pub struct AttackSettings {
    repeat: u16,
    intervale: f32,
}

impl Default for AttackSettings {
    fn default() -> Self {
        AttackSettings {
            repeat: 1,
            intervale: 1.0,
        }
    }
}

impl ReplayApp {
    pub fn attack_tab(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("attack_sdr_settings")
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
                    ui.label("TX Gain:");
                    ui.add_sized(egui::Vec2::new(45.0, 10.0), egui::TextEdit::singleline(&mut self.tx_gain));
                });
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.add_space(3.0);
                    let _apply_button = ui.button("Apply");
                });
            });
            ui.add_space(3.0);
        });

        egui::SidePanel::left("attack_settings")
            .min_width(150.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.heading("Attack Settings");
                ui.add_space(10.0);

                ui.label("Intervale:");
                let intervale_widget = egui::DragValue::new(&mut self.attack_settings.intervale).range(0.0..=60.0).speed(0.01).max_decimals(1);
                ui.add(intervale_widget);

                ui.label("Repeat:");
                let repeat_widget = egui::DragValue::new(&mut self.attack_settings.repeat).range(1..=65535).speed(1).max_decimals(0);
                ui.add(repeat_widget);
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
                let _load_button = ui.button("Load Signal");
                let _start_stop = ui.button("Start Attack");
            });
        });
    }
}

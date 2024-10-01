use crate::gui::app::ReplayApp;
use crate::gui::chart::signals_chart;
use crate::gui::sdr::sdr_ui;

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
    pub fn attack_tab(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("attack_sdr_settings")
        .max_height(65.0)
        .show_inside(ui, |ui| {
            sdr_ui(self, ui);
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
            signals_chart(ui);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                let _load_button = ui.button("Load Signal");
                let _start_stop = ui.button("Start Attack");
            });
        });
    }
}

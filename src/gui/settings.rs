use crate::gui::app::ReplayApp;

impl ReplayApp {
    pub fn settings_tab(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Text Size");
            let slider = egui::Slider::new(&mut self.temp_text_size, 0.1..=3.0)
                .step_by(0.05)
                .clamp_to_range(true);
            let slider = ui.add(slider);
            if slider.drag_stopped() {
                self.text_size = self.temp_text_size;
            } else if slider.changed() && !slider.dragged() {
                self.text_size = self.temp_text_size;
            }
        });
    }
}

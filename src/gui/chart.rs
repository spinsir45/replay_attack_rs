
pub fn signals_chart(ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        let sin: egui_plot::PlotPoints = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        }).collect();
        let line = egui_plot::Line::new(sin);
        egui_plot::Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
    });
}

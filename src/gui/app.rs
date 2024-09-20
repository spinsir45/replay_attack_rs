use eframe::egui;

enum PageTab {
    Settings,
    Monitor,
    Attack,
    Signals,
}

pub struct ReplayApp {
    pub(crate) temp_text_size: f32,
    pub(crate) text_size: f32,
    current_tab: PageTab,
}

impl ReplayApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load saved values
        let text_size: Option<f32> = eframe::get_value(_cc.storage.unwrap(), "text_size");

        // Apply saved values
        let mut app = ReplayApp::default();
        app.text_size = text_size.unwrap_or_default();
        app.temp_text_size = text_size.unwrap_or_default();
        app
    }
}

impl Default for ReplayApp {
    fn default() -> Self {
        ReplayApp {
            temp_text_size: 1.0,
            text_size: 1.0,
            current_tab: PageTab::Monitor,
        }
    }
}

impl eframe::App for ReplayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Change text size
        ctx.set_pixels_per_point(self.text_size);

        // Tab Menu Bar
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let monitor_tab = ui.button("Monitor");
                let attack_tab = ui.button("Attack");
                let signals_tab = ui.button("Signals");
                let settings_tab = ui.button("Settings");

                if monitor_tab.clicked() {
                    self.current_tab = PageTab::Monitor
                }
                if attack_tab.clicked() {
                    self.current_tab = PageTab::Attack
                }
                if signals_tab.clicked() {
                    self.current_tab = PageTab::Signals
                }
                if settings_tab.clicked() {
                    self.current_tab = PageTab::Settings
                }
            });
        });

        // Main display area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                PageTab::Monitor => self.monitor_tab(ctx, ui),
                PageTab::Attack => self.attack_tab(ctx),
                PageTab::Settings => self.settings_tab(ctx),
                PageTab::Signals => self.signals_tab(ctx),
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        eframe::set_value(_storage, "text_size", &self.text_size);
    }
}

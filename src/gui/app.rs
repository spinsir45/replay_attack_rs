use eframe::egui;
use crate::gui::monitor::MonitorMode;

enum PageTab {
    Settings,
    Monitor,
    Attack,
    Signals,
}

struct SDRSettings {
    pub center_frequency: f32,
    pub sample_rate: f32,
    pub tx_gain: u8,
    pub rx_gain: u8,
}

impl Default for SDRSettings {
    fn default() -> Self {
        SDRSettings {
            center_frequency: 915.0,
            sample_rate: 15.0,
            tx_gain: 1,
            rx_gain: 1,
        }
    }
}

pub struct ReplayApp {
    // App Settings
    pub(crate) temp_text_size: f32,
    pub(crate) text_size: f32,
    current_tab: PageTab,

    // SDR Settings
    pub(crate) sdr_settings: SDRSettings,
    pub(crate) center_frequency: String, // mHz (1 mHz = 1000000 Hz)
    pub(crate) sample_rate: String,      // mHz
    pub(crate) tx_gain: String,
    pub(crate) rx_gain: String,

    // Attack Settings
    pub(crate) repeat: u16,
    pub(crate) intervale: f32, // seconds

    // Monitor Settings
    pub(crate) monitor_mode: MonitorMode,
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

            // SDR Settings
            sdr_settings: SDRSettings::default(),
            center_frequency: "915.0".to_string(),
            sample_rate: "15.0".to_string(),
            tx_gain: "1".to_string(),
            rx_gain: "1".to_string(),

            // Attack Settings
            repeat: 0,
            intervale: 5.0,

            // Monitor Settings
            monitor_mode: MonitorMode::Manual,
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

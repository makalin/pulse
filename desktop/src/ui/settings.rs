use eframe::egui;
use crate::config::Config;

pub struct SettingsScreen {
    config: Config,
    api_url: String,
    theme: String,
    notifications_enabled: bool,
    auto_encrypt: bool,
}

impl SettingsScreen {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            api_url: config.api_url.clone(),
            theme: config.theme.clone(),
            notifications_enabled: config.notifications_enabled,
            auto_encrypt: config.auto_encrypt,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<Config> {
        let mut result = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Settings");
            ui.add_space(20.0);

            // API Settings
            ui.collapsing("API Settings", |ui| {
                ui.label("API URL:");
                ui.text_edit_singleline(&mut self.api_url);
            });

            // Appearance
            ui.collapsing("Appearance", |ui| {
                ui.label("Theme:");
                egui::ComboBox::from_label("")
                    .selected_text(&self.theme)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.theme, "Light".to_string(), "Light");
                        ui.selectable_value(&mut self.theme, "Dark".to_string(), "Dark");
                        ui.selectable_value(&mut self.theme, "System".to_string(), "System");
                    });
            });

            // Privacy
            ui.collapsing("Privacy", |ui| {
                ui.checkbox(&mut self.auto_encrypt, "Automatically encrypt messages");
                if ui.button("Export Encryption Keys").clicked() {
                    // TODO: Implement key export
                }
            });

            // Notifications
            ui.collapsing("Notifications", |ui| {
                ui.checkbox(&mut self.notifications_enabled, "Enable notifications");
            });

            ui.add_space(20.0);

            // Save/Cancel buttons
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    let new_config = Config {
                        api_url: self.api_url.clone(),
                        theme: self.theme.clone(),
                        notifications_enabled: self.notifications_enabled,
                        auto_encrypt: self.auto_encrypt,
                    };
                    result = Some(new_config);
                }

                if ui.button("Cancel").clicked() {
                    // Reset to original values
                    self.api_url = self.config.api_url.clone();
                    self.theme = self.config.theme.clone();
                    self.notifications_enabled = self.config.notifications_enabled;
                    self.auto_encrypt = self.config.auto_encrypt;
                }
            });
        });

        result
    }
} 
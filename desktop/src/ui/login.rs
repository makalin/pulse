use eframe::egui;
use crate::api::ApiClient;
use crate::app::User;

pub struct LoginScreen {
    email: String,
    password: String,
    error: Option<String>,
}

impl LoginScreen {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            error: None,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, api_client: &mut ApiClient) -> Option<User> {
        let mut result = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Pulse Login");
                ui.add_space(20.0);

                if let Some(error) = &self.error {
                    ui.colored_label(egui::Color32::RED, error);
                    ui.add_space(10.0);
                }

                ui.label("Email:");
                ui.text_edit_singleline(&mut self.email);
                ui.add_space(10.0);

                ui.label("Password:");
                ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                ui.add_space(20.0);

                if ui.button("Login").clicked() {
                    match api_client.login(&self.email, &self.password) {
                        Ok(user) => {
                            result = Some(user);
                            self.error = None;
                        }
                        Err(e) => {
                            self.error = Some(e.to_string());
                        }
                    }
                }

                ui.add_space(10.0);
                if ui.button("Create Account").clicked() {
                    // TODO: Show registration screen
                }
            });
        });

        result
    }
} 
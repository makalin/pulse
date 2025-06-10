use eframe::egui;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    ui::{
        login::LoginScreen,
        chat::ChatScreen,
        settings::SettingsScreen,
    },
    api::ApiClient,
    config::Config,
    crypto::CryptoManager,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub is_encrypted: bool,
}

pub enum Screen {
    Login,
    Chat,
    Settings,
}

pub struct PulseApp {
    screen: Screen,
    user: Option<User>,
    messages: Vec<Message>,
    api_client: ApiClient,
    config: Config,
    crypto: CryptoManager,
}

impl PulseApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load().unwrap_or_default();
        let api_client = ApiClient::new(&config.api_url);
        let crypto = CryptoManager::new();

        Self {
            screen: Screen::Login,
            user: None,
            messages: Vec::new(),
            api_client,
            config,
            crypto,
        }
    }
}

impl eframe::App for PulseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Login => {
                let login_screen = LoginScreen::new();
                if let Some(user) = login_screen.show(ctx, &mut self.api_client) {
                    self.user = Some(user);
                    self.screen = Screen::Chat;
                }
            }
            Screen::Chat => {
                let chat_screen = ChatScreen::new(
                    self.user.as_ref().unwrap(),
                    &self.messages,
                    &self.crypto,
                );
                if let Some(new_message) = chat_screen.show(ctx, &mut self.api_client) {
                    self.messages.push(new_message);
                }
            }
            Screen::Settings => {
                let settings_screen = SettingsScreen::new(&self.config);
                if let Some(new_config) = settings_screen.show(ctx) {
                    self.config = new_config;
                    self.config.save().unwrap();
                }
            }
        }

        // Show top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Settings").clicked() {
                        self.screen = Screen::Settings;
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                if let Some(user) = &self.user {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Logout").clicked() {
                            self.user = None;
                            self.screen = Screen::Login;
                        }
                        ui.label(format!("Logged in as: {}", user.username));
                    });
                }
            });
        });
    }
} 
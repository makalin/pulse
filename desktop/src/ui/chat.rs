use eframe::egui;
use crate::api::ApiClient;
use crate::app::{User, Message};
use crate::crypto::CryptoManager;
use uuid::Uuid;

pub struct ChatScreen {
    user: User,
    messages: Vec<Message>,
    new_message: String,
    selected_contact: Option<Uuid>,
    crypto: CryptoManager,
}

impl ChatScreen {
    pub fn new(user: &User, messages: &[Message], crypto: &CryptoManager) -> Self {
        Self {
            user: user.clone(),
            messages: messages.to_vec(),
            new_message: String::new(),
            selected_contact: None,
            crypto: crypto.clone(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, api_client: &mut ApiClient) -> Option<Message> {
        let mut result = None;

        egui::SidePanel::left("contacts_panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Contacts");
                ui.separator();
                
                // TODO: Add contact list
                if ui.button("Add Contact").clicked() {
                    // TODO: Show add contact dialog
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(contact_id) = self.selected_contact {
                // Chat view
                egui::ScrollArea::vertical()
                    .id_source("chat_messages")
                    .show(ui, |ui| {
                        for message in &self.messages {
                            let is_own = message.sender_id == self.user.id;
                            let alignment = if is_own {
                                egui::Align::RIGHT
                            } else {
                                egui::Align::LEFT
                            };

                            ui.with_layout(egui::Layout::top_down(alignment), |ui| {
                                let mut text = message.content.clone();
                                if message.is_encrypted {
                                    if let Ok(decrypted) = self.crypto.decrypt_message(&text) {
                                        text = decrypted;
                                    }
                                }

                                let response = ui.add(egui::TextEdit::multiline(&mut text)
                                    .frame(true)
                                    .interactive(false));

                                if response.hovered() {
                                    ui.label(message.timestamp.format("%H:%M").to_string());
                                }
                            });
                        }
                    });

                ui.separator();

                // Message input
                ui.horizontal(|ui| {
                    let response = ui.add(egui::TextEdit::multiline(&mut self.new_message)
                        .hint_text("Type a message...")
                        .desired_width(f32::INFINITY));

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if !self.new_message.trim().is_empty() {
                            let encrypted = self.crypto.encrypt_message(&self.new_message)
                                .unwrap_or_else(|_| self.new_message.clone());

                            let message = Message {
                                id: Uuid::new_v4(),
                                sender_id: self.user.id,
                                content: encrypted,
                                timestamp: chrono::Utc::now(),
                                is_encrypted: true,
                            };

                            if let Ok(_) = api_client.send_message(contact_id, &message) {
                                result = Some(message);
                                self.new_message.clear();
                            }
                        }
                    }

                    if ui.button("Send").clicked() {
                        if !self.new_message.trim().is_empty() {
                            let encrypted = self.crypto.encrypt_message(&self.new_message)
                                .unwrap_or_else(|_| self.new_message.clone());

                            let message = Message {
                                id: Uuid::new_v4(),
                                sender_id: self.user.id,
                                content: encrypted,
                                timestamp: chrono::Utc::now(),
                                is_encrypted: true,
                            };

                            if let Ok(_) = api_client.send_message(contact_id, &message) {
                                result = Some(message);
                                self.new_message.clear();
                            }
                        }
                    }
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Select a contact to start chatting");
                });
            }
        });

        result
    }
} 
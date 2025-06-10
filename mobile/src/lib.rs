use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

mod api;
mod crypto;
mod config;
mod storage;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: Uuid,
    pub name: Option<String>,
    pub is_group: bool,
    pub last_message: Option<Message>,
}

pub struct PulseMobile {
    api_client: api::ApiClient,
    crypto: crypto::CryptoManager,
    config: config::Config,
    storage: storage::Storage,
}

impl PulseMobile {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = config::Config::load()?;
        let api_client = api::ApiClient::new(&config.api_url);
        let crypto = crypto::CryptoManager::new()?;
        let storage = storage::Storage::new()?;

        Ok(Self {
            api_client,
            crypto,
            config,
            storage,
        })
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<User, Box<dyn std::error::Error>> {
        let user = self.api_client.login(email, password).await?;
        self.storage.save_user(&user)?;
        Ok(user)
    }

    pub async fn send_message(&self, recipient_id: Uuid, content: &str) -> Result<Message, Box<dyn std::error::Error>> {
        let encrypted = if self.config.auto_encrypt {
            self.crypto.encrypt_message(content)?
        } else {
            content.to_string()
        };

        let message = Message {
            id: Uuid::new_v4(),
            sender_id: self.storage.get_current_user()?.id,
            content: encrypted,
            timestamp: Utc::now(),
            is_encrypted: self.config.auto_encrypt,
        };

        self.api_client.send_message(recipient_id, &message).await?;
        self.storage.save_message(&message)?;
        Ok(message)
    }

    pub async fn get_messages(&self, chat_id: Uuid) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
        let messages = self.api_client.get_messages(chat_id, 50).await?;
        for message in &messages {
            self.storage.save_message(message)?;
        }
        Ok(messages)
    }

    pub async fn get_chats(&self) -> Result<Vec<Chat>, Box<dyn std::error::Error>> {
        self.api_client.get_chats().await
    }

    pub fn decrypt_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.crypto.decrypt_message(message)
    }

    pub fn update_config(&mut self, config: config::Config) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config;
        self.config.save()?;
        Ok(())
    }
}

// Flutter bindings
#[frb(mirror(User))]
pub struct _User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub public_key: Vec<u8>,
}

#[frb(mirror(Message))]
pub struct _Message {
    pub id: String,
    pub sender_id: String,
    pub content: String,
    pub timestamp: String,
    pub is_encrypted: bool,
}

#[frb(mirror(Chat))]
pub struct _Chat {
    pub id: String,
    pub name: Option<String>,
    pub is_group: bool,
    pub last_message: Option<_Message>,
}

#[frb(init)]
pub fn init_app() -> Result<PulseMobile, Box<dyn std::error::Error>> {
    PulseMobile::new()
} 
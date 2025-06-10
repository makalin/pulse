use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::app::{User, Message};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
}

pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            token: None,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<User, ApiError> {
        let response = self.client
            .post(&format!("{}/api/auth/login", self.base_url))
            .json(&serde_json::json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ApiError::ServerError(
                response.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            ));
        }

        let login_response: LoginResponse = response.json().await?;
        self.token = Some(login_response.token);
        Ok(login_response.user)
    }

    pub async fn send_message(&self, recipient_id: Uuid, message: &Message) -> Result<(), ApiError> {
        let response = self.client
            .post(&format!("{}/api/messages", self.base_url))
            .header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()))
            .json(&serde_json::json!({
                "recipient_id": recipient_id,
                "content": message.content,
                "is_encrypted": message.is_encrypted,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ApiError::ServerError(
                response.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            ));
        }

        Ok(())
    }

    pub async fn get_messages(&self, limit: i64) -> Result<Vec<Message>, ApiError> {
        let response = self.client
            .get(&format!("{}/api/messages?limit={}", self.base_url, limit))
            .header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ApiError::ServerError(
                response.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            ));
        }

        Ok(response.json().await?)
    }
}

#[derive(Debug, Deserialize)]
struct LoginResponse {
    token: String,
    user: User,
} 
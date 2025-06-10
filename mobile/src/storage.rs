use sqlx::{sqlite::SqlitePool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::{User, Message};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn new() -> Result<Self, StorageError> {
        let pool = SqlitePool::connect("sqlite:pulse.db").await?;
        Ok(Self { pool })
    }

    pub async fn init(&self) -> Result<(), StorageError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                email TEXT NOT NULL,
                public_key BLOB NOT NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                sender_id TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                is_encrypted BOOLEAN NOT NULL,
                FOREIGN KEY (sender_id) REFERENCES users(id)
            );

            CREATE TABLE IF NOT EXISTS chats (
                id TEXT PRIMARY KEY,
                name TEXT,
                is_group BOOLEAN NOT NULL,
                last_message_id TEXT,
                FOREIGN KEY (last_message_id) REFERENCES messages(id)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_user(&self, user: &User) -> Result<(), StorageError> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO users (id, username, email, public_key)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.public_key)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT * FROM users WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: Uuid::parse_str(r.get("id")).unwrap(),
            username: r.get("username"),
            email: r.get("email"),
            public_key: r.get("public_key"),
        }))
    }

    pub async fn save_message(&self, message: &Message) -> Result<(), StorageError> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO messages (id, sender_id, content, timestamp, is_encrypted)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(message.id.to_string())
        .bind(message.sender_id.to_string())
        .bind(&message.content)
        .bind(message.timestamp.to_rfc3339())
        .bind(message.is_encrypted)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_messages(&self, limit: i64) -> Result<Vec<Message>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM messages 
            ORDER BY timestamp DESC 
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Message {
                id: Uuid::parse_str(r.get("id")).unwrap(),
                sender_id: Uuid::parse_str(r.get("sender_id")).unwrap(),
                content: r.get("content"),
                timestamp: DateTime::parse_from_rfc3339(r.get("timestamp")).unwrap().with_timezone(&Utc),
                is_encrypted: r.get("is_encrypted"),
            })
            .collect())
    }

    pub async fn get_current_user(&self) -> Result<User, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT * FROM users LIMIT 1
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: Uuid::parse_str(row.get("id")).unwrap(),
            username: row.get("username"),
            email: row.get("email"),
            public_key: row.get("public_key"),
        })
    }
} 
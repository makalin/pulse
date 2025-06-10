use sqlx::{sqlite::SqlitePool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::{User, Message, Chat, ChatMember, Device, Session};

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DatabaseError> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn init(&self) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                public_key BLOB NOT NULL,
                created_at TEXT NOT NULL,
                last_seen TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                sender_id TEXT NOT NULL,
                recipient_id TEXT NOT NULL,
                content BLOB NOT NULL,
                associated_data BLOB,
                created_at TEXT NOT NULL,
                expires_at TEXT,
                FOREIGN KEY (sender_id) REFERENCES users(id),
                FOREIGN KEY (recipient_id) REFERENCES users(id)
            );

            CREATE TABLE IF NOT EXISTS chats (
                id TEXT PRIMARY KEY,
                name TEXT,
                is_group BOOLEAN NOT NULL,
                created_at TEXT NOT NULL,
                last_message_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS chat_members (
                chat_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                role TEXT NOT NULL,
                joined_at TEXT NOT NULL,
                PRIMARY KEY (chat_id, user_id),
                FOREIGN KEY (chat_id) REFERENCES chats(id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );

            CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                name TEXT NOT NULL,
                public_key BLOB NOT NULL,
                last_seen TEXT NOT NULL,
                is_online BOOLEAN NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                device_id TEXT NOT NULL,
                token TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id),
                FOREIGN KEY (device_id) REFERENCES devices(id)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // User operations
    pub async fn create_user(&self, user: &User) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, public_key, created_at, last_seen)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.public_key)
        .bind(user.created_at.to_rfc3339())
        .bind(user.last_seen.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>, DatabaseError> {
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
            created_at: DateTime::parse_from_rfc3339(r.get("created_at")).unwrap().with_timezone(&Utc),
            last_seen: DateTime::parse_from_rfc3339(r.get("last_seen")).unwrap().with_timezone(&Utc),
        }))
    }

    // Message operations
    pub async fn create_message(&self, message: &Message) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            INSERT INTO messages (id, sender_id, recipient_id, content, associated_data, created_at, expires_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(message.id.to_string())
        .bind(message.sender_id.to_string())
        .bind(message.recipient_id.to_string())
        .bind(&message.content)
        .bind(&message.associated_data)
        .bind(message.created_at.to_rfc3339())
        .bind(message.expires_at.map(|dt| dt.to_rfc3339()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_messages(&self, user_id: Uuid, limit: i64) -> Result<Vec<Message>, DatabaseError> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM messages 
            WHERE recipient_id = ? 
            ORDER BY created_at DESC 
            LIMIT ?
            "#,
        )
        .bind(user_id.to_string())
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Message {
                id: Uuid::parse_str(r.get("id")).unwrap(),
                sender_id: Uuid::parse_str(r.get("sender_id")).unwrap(),
                recipient_id: Uuid::parse_str(r.get("recipient_id")).unwrap(),
                content: r.get("content"),
                associated_data: r.get("associated_data"),
                created_at: DateTime::parse_from_rfc3339(r.get("created_at")).unwrap().with_timezone(&Utc),
                expires_at: r.get::<Option<String>, _>("expires_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
            })
            .collect())
    }

    // Session operations
    pub async fn create_session(&self, session: &Session) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            INSERT INTO sessions (id, user_id, device_id, token, created_at, expires_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(session.id.to_string())
        .bind(session.user_id.to_string())
        .bind(session.device_id.to_string())
        .bind(&session.token)
        .bind(session.created_at.to_rfc3339())
        .bind(session.expires_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn validate_session(&self, token: &str) -> Result<Option<Session>, DatabaseError> {
        let row = sqlx::query(
            r#"
            SELECT * FROM sessions 
            WHERE token = ? AND expires_at > datetime('now')
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Session {
            id: Uuid::parse_str(r.get("id")).unwrap(),
            user_id: Uuid::parse_str(r.get("user_id")).unwrap(),
            device_id: Uuid::parse_str(r.get("device_id")).unwrap(),
            token: r.get("token"),
            created_at: DateTime::parse_from_rfc3339(r.get("created_at")).unwrap().with_timezone(&Utc),
            expires_at: DateTime::parse_from_rfc3339(r.get("expires_at")).unwrap().with_timezone(&Utc),
        }))
    }
} 
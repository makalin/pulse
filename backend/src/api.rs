use axum::{
    routing::{get, post},
    Router,
    extract::{Path, State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use argon2::{self, Config};

use crate::{
    models::{User, Message, Session},
    db::Database,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // user id
    exp: usize,
    iat: usize,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
    public_key: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
    device_name: String,
    public_key: Vec<u8>,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Debug, Deserialize)]
struct SendMessageRequest {
    recipient_id: Uuid,
    content: Vec<u8>,
    associated_data: Option<Vec<u8>>,
    expires_at: Option<DateTime<Utc>>,
}

pub struct AppState {
    db: Database,
    jwt_secret: String,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/auth/login", post(login))
        .route("/api/messages", post(send_message))
        .route("/api/messages", get(get_messages))
        .with_state(state)
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // Hash password
    let salt = rand::random::<[u8; 16]>();
    let config = Config::default();
    let password_hash = argon2::hash_encoded(
        req.password.as_bytes(),
        &salt,
        &config,
    ).unwrap();

    let user = User {
        id: Uuid::new_v4(),
        username: req.username,
        email: req.email,
        public_key: req.public_key,
        created_at: Utc::now(),
        last_seen: Utc::now(),
    };

    match state.db.create_user(&user).await {
        Ok(_) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    // TODO: Verify password hash
    // TODO: Create device record

    let user = match state.db.get_user_by_email(&req.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let now = Utc::now();
    let exp = now + Duration::days(30);
    let claims = Claims {
        sub: user.id.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ).unwrap();

    let session = Session {
        id: Uuid::new_v4(),
        user_id: user.id,
        device_id: Uuid::new_v4(), // TODO: Use actual device ID
        token: token.clone(),
        created_at: now,
        expires_at: exp,
    };

    if let Err(e) = state.db.create_session(&session).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let response = LoginResponse {
        token,
        user,
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn send_message(
    State(state): State<AppState>,
    Json(req): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let message = Message {
        id: Uuid::new_v4(),
        sender_id: Uuid::new_v4(), // TODO: Get from auth token
        recipient_id: req.recipient_id,
        content: req.content,
        associated_data: req.associated_data,
        created_at: Utc::now(),
        expires_at: req.expires_at,
    };

    match state.db.create_message(&message).await {
        Ok(_) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_messages(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = Uuid::new_v4(); // TODO: Get from auth token
    match state.db.get_messages(user_id, 50).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
} 
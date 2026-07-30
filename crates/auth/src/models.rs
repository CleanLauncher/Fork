use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub username: String,
    pub uuid: String,
    pub user_type: AuthUserType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthUserType {
    Microsoft,
    Mojang,
    ElyBy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrosoftAuthConfig {
    pub client_id: String,
    pub redirect_uri: String,
    pub auth_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MojangAuthRequest {
    pub username: String,
    pub password: String,
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElyByAuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStorage {
    pub user_type: AuthUserType,
    pub access_token: String,
    pub refresh_token: String,
    pub username: String,
    pub uuid: String,
    pub expires_at: u64,
}

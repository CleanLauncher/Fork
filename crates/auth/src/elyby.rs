use error::{CoreError, Result};
use serde_json::Value;

use crate::models::{AuthState, AuthUserType};

pub struct ElyByAuth;

impl ElyByAuth {
    pub fn authenticate(username: &str, password: &str) -> Result<AuthState> {
        let body = serde_json::json!({
            "username": username,
            "password": password,
        });

        let resp = http_client::post_json(
            "https://authserver.ely.by/auth/authenticate",
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 200 {
            let error_body = String::from_utf8_lossy(&resp.body);
            return Err(CoreError::Http(format!(
                "Ely.by auth failed (HTTP {}): {}",
                resp.status, error_body
            )));
        }

        let json: Value = serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        let access_token = json["accessToken"]
            .as_str()
            .ok_or_else(|| CoreError::Http("Missing accessToken".into()))?;
        let refresh_token = json["clientToken"].as_str().unwrap_or("");
        let selected_profile = &json["selectedProfile"];

        let username = selected_profile["name"]
            .as_str()
            .unwrap_or("Player")
            .to_string();
        let uuid = selected_profile["id"].as_str().unwrap_or("").to_string();

        Ok(AuthState {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            expires_in: 86400,
            username,
            uuid,
            user_type: AuthUserType::ElyBy,
        })
    }

    pub fn refresh(access_token: &str, client_token: &str) -> Result<AuthState> {
        let body = serde_json::json!({
            "accessToken": access_token,
            "clientToken": client_token,
        });

        let resp = http_client::post_json(
            "https://authserver.ely.by/auth/refresh",
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Ely.by refresh failed: HTTP {}",
                resp.status
            )));
        }

        let json: Value = serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        let new_access_token = json["accessToken"]
            .as_str()
            .ok_or_else(|| CoreError::Http("Missing accessToken".into()))?;
        let new_client_token = json["clientToken"].as_str().unwrap_or(client_token);
        let selected_profile = &json["selectedProfile"];

        let username = selected_profile["name"]
            .as_str()
            .unwrap_or("Player")
            .to_string();
        let uuid = selected_profile["id"].as_str().unwrap_or("").to_string();

        Ok(AuthState {
            access_token: new_access_token.to_string(),
            refresh_token: new_client_token.to_string(),
            expires_in: 86400,
            username,
            uuid,
            user_type: AuthUserType::ElyBy,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elyby_urls() {
        // Verify URLs are well-formed
        let url = "https://authserver.ely.by/auth/authenticate";
        assert!(url.starts_with("https://"));
        assert!(url.contains("ely.by"));
    }
}

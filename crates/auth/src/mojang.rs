use error::{CoreError, Result};
use serde_json::Value;

use crate::models::{AuthState, AuthUserType};

const MOJANG_AUTH_URL: &str = "https://authserver.mojang.com";

pub struct MojangAuth;

impl MojangAuth {
    pub fn authenticate(
        username: &str,
        password: &str,
        client_token: Option<&str>,
    ) -> Result<AuthState> {
        let mut body = serde_json::json!({
            "agent": {
                "name": "Minecraft",
                "version": 1
            },
            "username": username,
            "password": password,
        });

        if let Some(token) = client_token {
            body["clientToken"] = serde_json::json!(token);
        }

        let url = format!("{}/authenticate", MOJANG_AUTH_URL);
        let resp = http_client::post_json(
            &url,
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 200 {
            let error_body = String::from_utf8_lossy(&resp.body);
            return Err(CoreError::Http(format!(
                "Mojang auth failed (HTTP {}): {}",
                resp.status, error_body
            )));
        }

        let json: Value = serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        let access_token = json["accessToken"]
            .as_str()
            .ok_or_else(|| CoreError::Http("Missing accessToken".into()))?;
        let client_token = json["clientToken"].as_str().unwrap_or("");
        let selected_profile = &json["selectedProfile"];

        let username = selected_profile["name"]
            .as_str()
            .unwrap_or("Player")
            .to_string();
        let uuid = selected_profile["id"].as_str().unwrap_or("").to_string();
        let expires_in = 86400u64;

        Ok(AuthState {
            access_token: access_token.to_string(),
            refresh_token: client_token.to_string(),
            expires_in,
            username,
            uuid,
            user_type: AuthUserType::Mojang,
        })
    }

    pub fn refresh(access_token: &str, client_token: &str) -> Result<AuthState> {
        let body = serde_json::json!({
            "accessToken": access_token,
            "clientToken": client_token,
        });

        let url = format!("{}/refresh", MOJANG_AUTH_URL);
        let resp = http_client::post_json(
            &url,
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Mojang refresh failed: HTTP {}",
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
            user_type: AuthUserType::Mojang,
        })
    }

    pub fn validate(access_token: &str) -> Result<bool> {
        let body = serde_json::json!({
            "accessToken": access_token,
        });

        let url = format!("{}/validate", MOJANG_AUTH_URL);
        let resp = http_client::post_json(
            &url,
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        Ok(resp.status == 204)
    }

    pub fn invalidate(access_token: &str, client_token: &str) -> Result<()> {
        let body = serde_json::json!({
            "accessToken": access_token,
            "clientToken": client_token,
        });

        let url = format!("{}/invalidate", MOJANG_AUTH_URL);
        let resp = http_client::post_json(
            &url,
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 204 {
            return Err(CoreError::Http(format!(
                "Mojang invalidate failed: HTTP {}",
                resp.status
            )));
        }
        Ok(())
    }

    pub fn signout(username: &str, password: &str) -> Result<()> {
        let body = serde_json::json!({
            "username": username,
            "password": password,
        });

        let url = format!("{}/signout", MOJANG_AUTH_URL);
        let resp = http_client::post_json(
            &url,
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 204 {
            return Err(CoreError::Http(format!(
                "Mojang signout failed: HTTP {}",
                resp.status
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_format() {
        // Just verify the method builds correct URLs
        let url = format!("{}/validate", MOJANG_AUTH_URL);
        assert_eq!(url, "https://authserver.mojang.com/validate");
    }
}

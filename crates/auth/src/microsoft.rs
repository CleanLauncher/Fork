use error::{CoreError, Result};
use serde_json::Value;

use crate::models::{AuthState, AuthUserType};

const MICROSOFT_AUTHORIZE_URL: &str = "https://login.live.com/oauth20_authorize.srf";
const MICROSOFT_TOKEN_URL: &str = "https://login.live.com/oauth20_token.srf";
const XBL_AUTH_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTH_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MINECRAFT_LOGIN_URL: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const MINECRAFT_PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

pub struct MicrosoftAuth {
    client_id: String,
    redirect_uri: String,
}

impl MicrosoftAuth {
    pub fn new(client_id: &str, redirect_uri: &str) -> Self {
        MicrosoftAuth {
            client_id: client_id.to_string(),
            redirect_uri: redirect_uri.to_string(),
        }
    }

    pub fn authorize_url(&self, state: &str) -> String {
        format!(
            "{}?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access&state={}",
            MICROSOFT_AUTHORIZE_URL,
            urlencode(&self.client_id),
            urlencode(&self.redirect_uri),
            urlencode(state)
        )
    }

    pub fn exchange_code(&self, auth_code: &str) -> Result<AuthState> {
        let body = serde_urlencoded::to_string([
            ("client_id", self.client_id.as_str()),
            ("code", auth_code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.redirect_uri.as_str()),
        ])
        .map_err(|e| CoreError::Http(e.to_string()))?;

        let token_response = self.post_form(MICROSOFT_TOKEN_URL, &body)?;
        let access_token = token_response["access_token"]
            .as_str()
            .ok_or_else(|| CoreError::Http("Missing access_token".into()))?;
        let refresh_token = token_response["refresh_token"]
            .as_str()
            .ok_or_else(|| CoreError::Http("Missing refresh_token".into()))?;
        let expires_in = token_response["expires_in"].as_u64().unwrap_or(3600);

        let xbl_token = self.xbl_authenticate(access_token)?;
        let xsts_data = self.xsts_authorize(&xbl_token)?;

        let mc_token = self.minecraft_login(&xsts_data)?;
        let profile = self.minecraft_profile(&mc_token)?;

        let username = profile["name"].as_str().unwrap_or("Player").to_string();
        let uuid = profile["id"].as_str().unwrap_or("").to_string();

        Ok(AuthState {
            access_token: mc_token,
            refresh_token: refresh_token.to_string(),
            expires_in,
            username,
            uuid,
            user_type: AuthUserType::Microsoft,
        })
    }

    pub fn refresh(&self, refresh_token: &str) -> Result<AuthState> {
        let body = serde_urlencoded::to_string([
            ("client_id", self.client_id.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
            ("redirect_uri", self.redirect_uri.as_str()),
        ])
        .map_err(|e| CoreError::Http(e.to_string()))?;

        let token_response = self.post_form(MICROSOFT_TOKEN_URL, &body)?;
        let new_access_token = token_response["access_token"]
            .as_str()
            .ok_or_else(|| CoreError::Http("Missing access_token".into()))?;
        let new_refresh_token = token_response["refresh_token"]
            .as_str()
            .unwrap_or(refresh_token);
        let expires_in = token_response["expires_in"].as_u64().unwrap_or(3600);

        let xbl_token = self.xbl_authenticate(new_access_token)?;
        let xsts_data = self.xsts_authorize(&xbl_token)?;
        let mc_token = self.minecraft_login(&xsts_data)?;
        let profile = self.minecraft_profile(&mc_token)?;

        let username = profile["name"].as_str().unwrap_or("Player").to_string();
        let uuid = profile["id"].as_str().unwrap_or("").to_string();

        Ok(AuthState {
            access_token: mc_token,
            refresh_token: new_refresh_token.to_string(),
            expires_in,
            username,
            uuid,
            user_type: AuthUserType::Microsoft,
        })
    }

    fn xbl_authenticate(&self, access_token: &str) -> Result<String> {
        let body = serde_json::json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": format!("d={}", access_token)
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        });

        let resp = http_client::post_json(
            XBL_AUTH_URL,
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "XBL auth failed: HTTP {}",
                resp.status
            )));
        }

        let json: Value = serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        json["Token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| CoreError::Http("Missing XBL token".into()))
    }

    fn xsts_authorize(&self, xbl_token: &str) -> Result<String> {
        let body = serde_json::json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbl_token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        });

        let resp = http_client::post_json(
            XSTS_AUTH_URL,
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "XSTS auth failed: HTTP {}",
                resp.status
            )));
        }

        let json: Value = serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let token = json["Token"]
            .as_str()
            .ok_or_else(|| CoreError::Http("Missing XSTS token".into()))?;
        let uhs = json["DisplayClaims"]["xui"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|xui| xui["uhs"].as_str())
            .ok_or_else(|| CoreError::Http("Missing uhs".into()))?;

        Ok(format!("{};{}", uhs, token))
    }

    fn minecraft_login(&self, xsts_data: &str) -> Result<String> {
        let body = serde_json::json!({
            "identityToken": format!("XBL3.0 x={}", xsts_data)
        });

        let resp = http_client::post_json(
            MINECRAFT_LOGIN_URL,
            &serde_json::to_vec(&body).map_err(CoreError::Json)?,
            None,
        )?;

        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Minecraft login failed: HTTP {}",
                resp.status
            )));
        }

        let json: Value = serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        json["access_token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| CoreError::Http("Missing Minecraft access_token".into()))
    }

    fn minecraft_profile(&self, mc_token: &str) -> Result<Value> {
        let resp = http_client::get_with_headers(
            MINECRAFT_PROFILE_URL,
            &[("Authorization".to_string(), format!("Bearer {}", mc_token))],
        )?;

        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Profile fetch failed: HTTP {}",
                resp.status
            )));
        }

        serde_json::from_slice(&resp.body).map_err(CoreError::Json)
    }

    fn post_form(&self, url: &str, body: &str) -> Result<Value> {
        let resp = http_client::post_json(url, body.as_bytes(), None)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Token exchange failed: HTTP {}",
                resp.status
            )));
        }
        serde_json::from_slice(&resp.body).map_err(CoreError::Json)
    }
}

fn urlencode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorize_url_format() {
        let auth = MicrosoftAuth::new("test-client", "http://localhost:8080");
        let url = auth.authorize_url("test-state");
        assert!(url.contains("client_id=test-client"));
        assert!(url.contains("redirect_uri=http%3A%2F%2Flocalhost%3A8080"));
        assert!(url.contains("state=test-state"));
        assert!(url.contains("XboxLive.signin"));
    }

    #[test]
    fn test_urlencode() {
        assert_eq!(urlencode("simple"), "simple");
        assert_eq!(urlencode("a b"), "a%20b");
        assert_eq!(urlencode("http://example.com"), "http%3A%2F%2Fexample.com");
    }
}

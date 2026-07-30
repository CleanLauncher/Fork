use std::collections::HashMap;
use std::sync::Mutex;

use error::{CoreError, Result};
use once_cell::sync::Lazy;

use crate::core::{get_with_headers, HttpResponse};

static API_KEYS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_api_key(platform: &str, api_key: &str) {
    if let Ok(mut keys) = API_KEYS.lock() {
        keys.insert(platform.to_string(), api_key.to_string());
    }
}

pub fn get_api_key(platform: &str) -> Option<String> {
    API_KEYS
        .lock()
        .ok()
        .and_then(|keys| keys.get(platform).cloned())
}

pub fn fetch_with_api_key(url: &str, platform: &str) -> Result<HttpResponse> {
    let api_key = get_api_key(platform)
        .ok_or_else(|| CoreError::Http(format!("No API key for {}", platform)))?;
    get_with_headers(url, &[("x-api-key".to_string(), api_key)])
}

pub fn fetch_json(url: &str) -> Result<serde_json::Value> {
    let resp = crate::core::get(url)?;
    if resp.status != 200 {
        return Err(CoreError::Http(format!("HTTP {}", resp.status)));
    }
    serde_json::from_slice(&resp.body).map_err(CoreError::Json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_get_api_key() {
        register_api_key("test", "key123");
        assert_eq!(get_api_key("test"), Some("key123".into()));
    }
}

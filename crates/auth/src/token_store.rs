use std::fs;
use std::path::PathBuf;

use error::{CoreError, Result};

use crate::models::{AuthState, AuthUserType, TokenStorage};

pub struct TokenStore {
    storage_dir: PathBuf,
}

impl TokenStore {
    pub fn new(storage_dir: &str) -> Self {
        let path = PathBuf::from(storage_dir);
        fs::create_dir_all(&path).ok();
        TokenStore { storage_dir: path }
    }

    pub fn save(&self, storage: &TokenStorage) -> Result<()> {
        let path = self.token_path(&storage.uuid);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(storage).map_err(CoreError::Json)?;
        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, &json)?;
        fs::rename(&temp_path, &path)?;
        Ok(())
    }

    pub fn load(&self, uuid: &str) -> Result<TokenStorage> {
        let path = self.token_path(uuid);
        if !path.exists() {
            return Err(CoreError::InvalidData("Token not found".into()));
        }
        let content = fs::read_to_string(&path)?;
        let storage: TokenStorage = serde_json::from_str(&content).map_err(CoreError::Json)?;
        Ok(storage)
    }

    pub fn load_all(&self) -> Result<Vec<TokenStorage>> {
        let mut accounts = Vec::new();
        if !self.storage_dir.exists() {
            return Ok(accounts);
        }
        for entry in fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(storage) = serde_json::from_str::<TokenStorage>(&content) {
                        accounts.push(storage);
                    }
                }
            }
        }
        Ok(accounts)
    }

    pub fn remove(&self, uuid: &str) -> Result<()> {
        let path = self.token_path(uuid);
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }

    pub fn has_valid_token(&self, uuid: &str) -> bool {
        self.load(uuid)
            .map(|s| {
                std::time::UNIX_EPOCH
                    .elapsed()
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
                    < s.expires_at
            })
            .unwrap_or(false)
    }

    pub fn to_auth_state(&self, storage: &TokenStorage) -> AuthState {
        let now = std::time::UNIX_EPOCH
            .elapsed()
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let expires_in = storage.expires_at.saturating_sub(now);
        AuthState {
            access_token: storage.access_token.clone(),
            refresh_token: storage.refresh_token.clone(),
            expires_in,
            username: storage.username.clone(),
            uuid: storage.uuid.clone(),
            user_type: storage.user_type,
        }
    }

    fn token_path(&self, uuid: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.json", uuid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_load_roundtrip() {
        let dir = std::env::temp_dir().join("auth_token_test");
        fs::remove_dir_all(&dir).ok();
        let store = TokenStore::new(dir.to_str().unwrap());

        let storage = TokenStorage {
            user_type: AuthUserType::Microsoft,
            access_token: "test_access".into(),
            refresh_token: "test_refresh".into(),
            username: "TestUser".into(),
            uuid: "550e8400-e29b-41d4-a716-446655440000".into(),
            expires_at: 9999999999,
        };

        store.save(&storage).unwrap();
        let loaded = store.load(&storage.uuid).unwrap();
        assert_eq!(loaded.username, "TestUser");
        assert_eq!(loaded.access_token, "test_access");
        assert_eq!(loaded.user_type, AuthUserType::Microsoft);

        let all = store.load_all().unwrap();
        assert_eq!(all.len(), 1);

        store.remove(&storage.uuid).unwrap();
        assert!(store.load(&storage.uuid).is_err());
        fs::remove_dir_all(&dir).ok();
    }
}

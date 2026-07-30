use error::{CoreError, Result};

use crate::models::{ApiClient, ModpackEntry, ModpackPlatform, ModpackVersion};

pub struct CurseForgeClient {
    api_key: String,
}

impl CurseForgeClient {
    pub fn new(api_key: &str) -> Self {
        CurseForgeClient {
            api_key: api_key.to_string(),
        }
    }

    fn api_url(path: &str) -> String {
        format!("https://api.curseforge.com/v1{}", path)
    }

    fn auth_header(&self) -> (String, String) {
        ("x-api-key".to_string(), self.api_key.clone())
    }
}

impl ApiClient for CurseForgeClient {
    fn platform(&self) -> ModpackPlatform {
        ModpackPlatform::CurseForge
    }

    fn search_packs(&self, query: &str, limit: u32) -> Result<Vec<ModpackEntry>> {
        let url = Self::api_url(&format!(
            "/mods/search?gameId=432&classId=4471&searchFilter={}&pageSize={}",
            urlencode(query),
            limit
        ));

        let resp = http_client::get_with_headers(&url, &[self.auth_header()])?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "CurseForge search failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let data = json["data"]
            .as_array()
            .ok_or_else(|| CoreError::InvalidData("Missing data".into()))?;

        data.iter()
            .map(|item| {
                Ok(ModpackEntry {
                    id: item["id"].as_i64().unwrap_or(0).to_string(),
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    slug: item["slug"].as_str().unwrap_or("").to_string(),
                    description: item["summary"].as_str().unwrap_or("").to_string(),
                    author: item["authors"]
                        .as_array()
                        .and_then(|arr| arr.first())
                        .and_then(|a| a["name"].as_str())
                        .unwrap_or("")
                        .to_string(),
                    icon_url: item["logo"]
                        .as_object()
                        .and_then(|logo| logo["thumbnailUrl"].as_str())
                        .unwrap_or("")
                        .to_string(),
                    mod_count: 0,
                    download_count: item["downloadCount"].as_u64().unwrap_or(0),
                    platform: ModpackPlatform::CurseForge,
                })
            })
            .collect()
    }

    fn get_pack(&self, pack_id: &str) -> Result<ModpackEntry> {
        let url = Self::api_url(&format!("/mods/{}", pack_id));
        let resp = http_client::get_with_headers(&url, &[self.auth_header()])?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "CurseForge get pack failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let item = &json["data"];

        Ok(ModpackEntry {
            id: item["id"].as_i64().unwrap_or(0).to_string(),
            name: item["name"].as_str().unwrap_or("").to_string(),
            slug: item["slug"].as_str().unwrap_or("").to_string(),
            description: item["summary"].as_str().unwrap_or("").to_string(),
            author: item["authors"]
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|a| a["name"].as_str())
                .unwrap_or("")
                .to_string(),
            icon_url: item["logo"]
                .as_object()
                .and_then(|logo| logo["thumbnailUrl"].as_str())
                .unwrap_or("")
                .to_string(),
            mod_count: 0,
            download_count: item["downloadCount"].as_u64().unwrap_or(0),
            platform: ModpackPlatform::CurseForge,
        })
    }

    fn get_versions(&self, pack_id: &str) -> Result<Vec<ModpackVersion>> {
        let url = Self::api_url(&format!("/mods/{}/files", pack_id));
        let resp = http_client::get_with_headers(&url, &[self.auth_header()])?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "CurseForge get versions failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let data = json["data"]
            .as_array()
            .ok_or_else(|| CoreError::InvalidData("Missing data".into()))?;

        data.iter()
            .map(|file| {
                let game_versions = file["gameVersions"]
                    .as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                Ok(ModpackVersion {
                    id: file["id"].as_i64().unwrap_or(0).to_string(),
                    name: file["displayName"].as_str().unwrap_or("").to_string(),
                    version_number: file["fileName"].as_str().unwrap_or("").to_string(),
                    minecraft_version: game_versions,
                    loader: "".to_string(),
                    download_url: file["downloadUrl"].as_str().unwrap_or("").to_string(),
                    file_size: file["fileLength"].as_u64().unwrap_or(0),
                    release_date: file["fileDate"].as_str().unwrap_or("").to_string(),
                    version_type: match file["releaseType"].as_i64().unwrap_or(1) {
                        1 => "release",
                        2 => "beta",
                        3 => "alpha",
                        _ => "unknown",
                    }
                    .to_string(),
                })
            })
            .collect()
    }

    fn get_version(&self, pack_id: &str, version_id: &str) -> Result<ModpackVersion> {
        let url = Self::api_url(&format!("/mods/{}/files/{}", pack_id, version_id));
        let resp = http_client::get_with_headers(&url, &[self.auth_header()])?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "CurseForge get version failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let file = &json["data"];

        let game_versions = file["gameVersions"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Ok(ModpackVersion {
            id: file["id"].as_i64().unwrap_or(0).to_string(),
            name: file["displayName"].as_str().unwrap_or("").to_string(),
            version_number: file["fileName"].as_str().unwrap_or("").to_string(),
            minecraft_version: game_versions,
            loader: "".to_string(),
            download_url: file["downloadUrl"].as_str().unwrap_or("").to_string(),
            file_size: file["fileLength"].as_u64().unwrap_or(0),
            release_date: file["fileDate"].as_str().unwrap_or("").to_string(),
            version_type: match file["releaseType"].as_i64().unwrap_or(1) {
                1 => "release",
                2 => "beta",
                3 => "alpha",
                _ => "unknown",
            }
            .to_string(),
        })
    }

    fn get_latest_version(
        &self,
        pack_id: &str,
        mc_version: &str,
        _loader: &str,
    ) -> Result<Option<ModpackVersion>> {
        let url = Self::api_url(&format!(
            "/mods/{}/files/latest?gameVersion={}",
            pack_id,
            urlencode(mc_version)
        ));
        let resp = http_client::get_with_headers(&url, &[self.auth_header()])?;
        if resp.status != 200 {
            return Ok(None);
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let file = &json["data"];

        let game_versions = file["gameVersions"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Ok(Some(ModpackVersion {
            id: file["id"].as_i64().unwrap_or(0).to_string(),
            name: file["displayName"].as_str().unwrap_or("").to_string(),
            version_number: file["fileName"].as_str().unwrap_or("").to_string(),
            minecraft_version: game_versions,
            loader: "".to_string(),
            download_url: file["downloadUrl"].as_str().unwrap_or("").to_string(),
            file_size: file["fileLength"].as_u64().unwrap_or(0),
            release_date: file["fileDate"].as_str().unwrap_or("").to_string(),
            version_type: match file["releaseType"].as_i64().unwrap_or(1) {
                1 => "release",
                2 => "beta",
                3 => "alpha",
                _ => "unknown",
            }
            .to_string(),
        }))
    }

    fn download_url(&self, pack_id: &str, version_id: &str) -> Result<String> {
        let version = self.get_version(pack_id, version_id)?;
        if version.download_url.is_empty() {
            return Err(CoreError::Http("No download URL available".into()));
        }
        Ok(version.download_url)
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
    fn test_api_url() {
        let url = CurseForgeClient::api_url("/mods/search");
        assert!(url.contains("api.curseforge.com"));
    }
}

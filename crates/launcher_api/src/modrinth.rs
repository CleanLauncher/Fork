use error::{CoreError, Result};

use crate::models::{ApiClient, ModpackEntry, ModpackPlatform, ModpackVersion};

pub struct ModrinthClient;

impl ModrinthClient {
    pub fn new() -> Self {
        ModrinthClient
    }

    fn api_url(path: &str) -> String {
        format!("https://api.modrinth.com/v2{}", path)
    }
}

impl ApiClient for ModrinthClient {
    fn platform(&self) -> ModpackPlatform {
        ModpackPlatform::Modrinth
    }

    fn search_packs(&self, query: &str, limit: u32) -> Result<Vec<ModpackEntry>> {
        let url = Self::api_url(&format!(
            "/search?query={}&limit={}&facets=[[%22project_type:modpack%22]]",
            urlencode(query),
            limit
        ));

        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Modrinth search failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let hits = json["hits"]
            .as_array()
            .ok_or_else(|| CoreError::InvalidData("Missing hits".into()))?;

        hits.iter()
            .map(|hit| {
                Ok(ModpackEntry {
                    id: hit["project_id"].as_str().unwrap_or("").to_string(),
                    name: hit["title"].as_str().unwrap_or("").to_string(),
                    slug: hit["slug"].as_str().unwrap_or("").to_string(),
                    description: hit["description"].as_str().unwrap_or("").to_string(),
                    author: hit["author"].as_str().unwrap_or("").to_string(),
                    icon_url: hit["icon_url"].as_str().unwrap_or("").to_string(),
                    mod_count: hit["mod_count"].as_u64().unwrap_or(0) as u32,
                    download_count: hit["downloads"].as_u64().unwrap_or(0),
                    platform: ModpackPlatform::Modrinth,
                })
            })
            .collect()
    }

    fn get_pack(&self, pack_id: &str) -> Result<ModpackEntry> {
        let url = Self::api_url(&format!("/project/{}", pack_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Modrinth get pack failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        Ok(ModpackEntry {
            id: json["id"].as_str().unwrap_or("").to_string(),
            name: json["title"].as_str().unwrap_or("").to_string(),
            slug: json["slug"].as_str().unwrap_or("").to_string(),
            description: json["description"].as_str().unwrap_or("").to_string(),
            author: json["author"].as_str().unwrap_or("").to_string(),
            icon_url: json["icon_url"].as_str().unwrap_or("").to_string(),
            mod_count: json["mod_count"].as_u64().unwrap_or(0) as u32,
            download_count: json["downloads"].as_u64().unwrap_or(0),
            platform: ModpackPlatform::Modrinth,
        })
    }

    fn get_versions(&self, pack_id: &str) -> Result<Vec<ModpackVersion>> {
        let url = Self::api_url(&format!("/project/{}/version", pack_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Modrinth get versions failed: HTTP {}",
                resp.status
            )));
        }

        let versions: Vec<serde_json::Value> =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        versions
            .iter()
            .map(|v| {
                let game_versions = v["game_versions"]
                    .as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let loaders = v["loaders"]
                    .as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                Ok(ModpackVersion {
                    id: v["id"].as_str().unwrap_or("").to_string(),
                    name: v["name"].as_str().unwrap_or("").to_string(),
                    version_number: v["version_number"].as_str().unwrap_or("").to_string(),
                    minecraft_version: game_versions,
                    loader: loaders,
                    download_url: v["files"]
                        .as_array()
                        .and_then(|files| files.first())
                        .and_then(|f| f["url"].as_str())
                        .unwrap_or("")
                        .to_string(),
                    file_size: v["files"]
                        .as_array()
                        .and_then(|files| files.first())
                        .and_then(|f| f["size"].as_u64())
                        .unwrap_or(0),
                    release_date: v["date_published"].as_str().unwrap_or("").to_string(),
                    version_type: v["version_type"].as_str().unwrap_or("release").to_string(),
                })
            })
            .collect()
    }

    fn get_version(&self, pack_id: &str, version_id: &str) -> Result<ModpackVersion> {
        let url = Self::api_url(&format!("/project/{}/version/{}", pack_id, version_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Modrinth get version failed: HTTP {}",
                resp.status
            )));
        }

        let v: serde_json::Value = serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        let game_versions = v["game_versions"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let loaders = v["loaders"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Ok(ModpackVersion {
            id: v["id"].as_str().unwrap_or("").to_string(),
            name: v["name"].as_str().unwrap_or("").to_string(),
            version_number: v["version_number"].as_str().unwrap_or("").to_string(),
            minecraft_version: game_versions,
            loader: loaders,
            download_url: v["files"]
                .as_array()
                .and_then(|files| files.first())
                .and_then(|f| f["url"].as_str())
                .unwrap_or("")
                .to_string(),
            file_size: v["files"]
                .as_array()
                .and_then(|files| files.first())
                .and_then(|f| f["size"].as_u64())
                .unwrap_or(0),
            release_date: v["date_published"].as_str().unwrap_or("").to_string(),
            version_type: v["version_type"].as_str().unwrap_or("release").to_string(),
        })
    }

    fn get_latest_version(
        &self,
        pack_id: &str,
        mc_version: &str,
        loader: &str,
    ) -> Result<Option<ModpackVersion>> {
        let versions = self.get_versions(pack_id)?;
        let filtered: Vec<_> = versions
            .into_iter()
            .filter(|v| {
                let mc_match = mc_version.is_empty() || v.minecraft_version == mc_version;
                let loader_match = loader.is_empty() || v.loader == loader;
                mc_match && loader_match
            })
            .collect();

        Ok(filtered.into_iter().next())
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
    fn test_url_format() {
        let url = ModrinthClient::api_url("/search?query=test");
        assert!(url.contains("api.modrinth.com"));
        assert!(url.contains("/v2/search"));
    }

    #[test]
    fn test_urlencode() {
        assert_eq!(urlencode("hello world"), "hello%20world");
        assert_eq!(urlencode("test"), "test");
    }
}

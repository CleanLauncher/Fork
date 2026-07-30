use error::{CoreError, Result};

use crate::models::{ApiClient, ModpackEntry, ModpackPlatform, ModpackVersion};

pub struct ATLauncherClient;

impl ATLauncherClient {
    pub fn new() -> Self {
        ATLauncherClient
    }

    fn api_url(path: &str) -> String {
        format!("https://api.atlauncher.com/v1{}", path)
    }
}

impl ApiClient for ATLauncherClient {
    fn platform(&self) -> ModpackPlatform {
        ModpackPlatform::ATLauncher
    }

    fn search_packs(&self, query: &str, limit: u32) -> Result<Vec<ModpackEntry>> {
        let url = Self::api_url(&format!(
            "/packs/search?query={}&limit={}",
            urlencode(query),
            limit
        ));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "ATL search failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        if let Some(packs) = json["data"].as_array() {
            packs
                .iter()
                .map(|p| {
                    Ok(ModpackEntry {
                        id: p["id"].as_i64().unwrap_or(0).to_string(),
                        name: p["name"].as_str().unwrap_or("").to_string(),
                        slug: p["slug"].as_str().unwrap_or("").to_string(),
                        description: p["description"].as_str().unwrap_or("").to_string(),
                        author: p["developer"]["username"]
                            .as_str()
                            .unwrap_or("")
                            .to_string(),
                        icon_url: p["imageUrl"]
                            .or_else(|| p["iconUrl"])
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        mod_count: p["modCount"].as_u64().unwrap_or(0) as u32,
                        download_count: 0,
                        platform: ModpackPlatform::ATLauncher,
                    })
                })
                .collect()
        } else {
            Ok(Vec::new())
        }
    }

    fn get_pack(&self, pack_id: &str) -> Result<ModpackEntry> {
        let url = Self::api_url(&format!("/packs/{}", pack_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "ATL get pack failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let p = &json["data"];

        Ok(ModpackEntry {
            id: p["id"].as_i64().unwrap_or(0).to_string(),
            name: p["name"].as_str().unwrap_or("").to_string(),
            slug: p["slug"].as_str().unwrap_or("").to_string(),
            description: p["description"].as_str().unwrap_or("").to_string(),
            author: p["developer"]["username"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            icon_url: p["imageUrl"]
                .or_else(|| p["iconUrl"])
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            mod_count: p["modCount"].as_u64().unwrap_or(0) as u32,
            download_count: 0,
            platform: ModpackPlatform::ATLauncher,
        })
    }

    fn get_versions(&self, pack_id: &str) -> Result<Vec<ModpackVersion>> {
        let url = Self::api_url(&format!("/packs/{}/versions", pack_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "ATL get versions failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        if let Some(versions) = json["data"].as_array() {
            versions
                .iter()
                .map(|v| {
                    Ok(ModpackVersion {
                        id: v["id"].as_i64().unwrap_or(0).to_string(),
                        name: v["version"].as_str().unwrap_or("").to_string(),
                        version_number: v["version"].as_str().unwrap_or("").to_string(),
                        minecraft_version: v["minecraftVersion"].as_str().unwrap_or("").to_string(),
                        loader: v["loader"].as_str().unwrap_or("vanilla").to_string(),
                        download_url: v["downloadUrl"].as_str().unwrap_or("").to_string(),
                        file_size: v["filesize"].as_u64().unwrap_or(0),
                        release_date: v["publishedDate"].as_str().unwrap_or("").to_string(),
                        version_type: v["type"].as_str().unwrap_or("release").to_string(),
                    })
                })
                .collect()
        } else {
            Ok(Vec::new())
        }
    }

    fn get_version(&self, pack_id: &str, version_id: &str) -> Result<ModpackVersion> {
        let url = Self::api_url(&format!("/packs/{}/version/{}", pack_id, version_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "ATL get version failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let v = &json["data"];

        Ok(ModpackVersion {
            id: version_id.to_string(),
            name: v["version"].as_str().unwrap_or("").to_string(),
            version_number: v["version"].as_str().unwrap_or("").to_string(),
            minecraft_version: v["minecraftVersion"].as_str().unwrap_or("").to_string(),
            loader: v["loader"].as_str().unwrap_or("vanilla").to_string(),
            download_url: v["downloadUrl"].as_str().unwrap_or("").to_string(),
            file_size: v["filesize"].as_u64().unwrap_or(0),
            release_date: v["publishedDate"].as_str().unwrap_or("").to_string(),
            version_type: v["type"].as_str().unwrap_or("release").to_string(),
        })
    }

    fn get_latest_version(
        &self,
        pack_id: &str,
        _mc_version: &str,
        _loader: &str,
    ) -> Result<Option<ModpackVersion>> {
        let versions = self.get_versions(pack_id)?;
        Ok(versions.into_iter().next())
    }

    fn download_url(&self, _pack_id: &str, version_id: &str) -> Result<String> {
        // ATLauncher provides direct download URLs in version responses
        Err(CoreError::Http("Use get_version for download URL".into()))
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

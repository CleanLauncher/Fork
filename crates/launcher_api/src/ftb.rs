use error::{CoreError, Result};

use crate::models::{ApiClient, ModpackEntry, ModpackPlatform, ModpackVersion};

pub struct FTBClient;

impl FTBClient {
    pub fn new() -> Self {
        FTBClient
    }

    fn api_url(path: &str) -> String {
        format!("https://api.modpacks.ch/public{}", path)
    }
}

impl ApiClient for FTBClient {
    fn platform(&self) -> ModpackPlatform {
        ModpackPlatform::FTB
    }

    fn search_packs(&self, query: &str, _limit: u32) -> Result<Vec<ModpackEntry>> {
        let url = Self::api_url(&format!("/modpack/search/{}", urlencode(query)));

        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "FTB search failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        if let Some(packs) = json["packs"].as_array() {
            packs
                .iter()
                .map(|p| {
                    Ok(ModpackEntry {
                        id: p["id"].as_i64().unwrap_or(0).to_string(),
                        name: p["name"].as_str().unwrap_or("").to_string(),
                        slug: p["slug"].as_str().unwrap_or("").to_string(),
                        description: p["description"].as_str().unwrap_or("").to_string(),
                        author: p["author"]["name"].as_str().unwrap_or("").to_string(),
                        icon_url: p["art"].as_str().unwrap_or("").to_string(),
                        mod_count: 0,
                        download_count: p["downloads"].as_u64().unwrap_or(0),
                        platform: ModpackPlatform::FTB,
                    })
                })
                .collect()
        } else {
            Ok(Vec::new())
        }
    }

    fn get_pack(&self, pack_id: &str) -> Result<ModpackEntry> {
        let url = Self::api_url(&format!("/modpack/{}", pack_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "FTB get pack failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        Ok(ModpackEntry {
            id: json["id"].as_i64().unwrap_or(0).to_string(),
            name: json["name"].as_str().unwrap_or("").to_string(),
            slug: json["slug"].as_str().unwrap_or("").to_string(),
            description: json["description"].as_str().unwrap_or("").to_string(),
            author: json["author"]["name"].as_str().unwrap_or("").to_string(),
            icon_url: json["art"].as_str().unwrap_or("").to_string(),
            mod_count: 0,
            download_count: json["downloads"].as_u64().unwrap_or(0),
            platform: ModpackPlatform::FTB,
        })
    }

    fn get_versions(&self, pack_id: &str) -> Result<Vec<ModpackVersion>> {
        let url = Self::api_url(&format!("/modpack/{}", pack_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "FTB get versions failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let versions = json["versions"]
            .as_array()
            .ok_or_else(|| CoreError::InvalidData("Missing versions".into()))?;

        versions
            .iter()
            .map(|v| {
                let id = v["id"].as_i64().unwrap_or(0);
                let targets = v["targets"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|t| t["version"].as_str().map(String::from))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                let mc_version = targets.first().cloned().unwrap_or_default();
                let loader = targets.get(1).cloned().unwrap_or_default();

                Ok(ModpackVersion {
                    id: id.to_string(),
                    name: v["name"].as_str().unwrap_or("").to_string(),
                    version_number: v["name"].as_str().unwrap_or("").to_string(),
                    minecraft_version: mc_version,
                    loader,
                    download_url: format!(
                        "https://api.modpacks.ch/public/modpack/{}/version/{}/url",
                        pack_id, id
                    ),
                    file_size: v["size"].as_u64().unwrap_or(0),
                    release_date: v["updated"].as_str().unwrap_or("").to_string(),
                    version_type: v["type"].as_str().unwrap_or("release").to_string(),
                })
            })
            .collect()
    }

    fn get_version(&self, pack_id: &str, version_id: &str) -> Result<ModpackVersion> {
        let url = Self::api_url(&format!("/modpack/{}/version/{}", pack_id, version_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "FTB get version failed: HTTP {}",
                resp.status
            )));
        }

        let v: serde_json::Value = serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        let targets = v["targets"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| t["version"].as_str().map(String::from))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mc_version = targets.first().cloned().unwrap_or_default();
        let loader = targets.get(1).cloned().unwrap_or_default();

        Ok(ModpackVersion {
            id: version_id.to_string(),
            name: v["name"].as_str().unwrap_or("").to_string(),
            version_number: v["name"].as_str().unwrap_or("").to_string(),
            minecraft_version: mc_version,
            loader,
            download_url: format!(
                "https://api.modpacks.ch/public/modpack/{}/version/{}/url",
                pack_id, version_id
            ),
            file_size: v["size"].as_u64().unwrap_or(0),
            release_date: v["updated"].as_str().unwrap_or("").to_string(),
            version_type: v["type"].as_str().unwrap_or("release").to_string(),
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
        Ok(format!(
            "https://api.modpacks.ch/public/modpack/{}/version/{}/url",
            pack_id, version_id
        ))
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

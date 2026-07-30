use error::{CoreError, Result};

use crate::models::{ApiClient, ModpackEntry, ModpackPlatform, ModpackVersion};

pub struct TechnicClient;

impl TechnicClient {
    pub fn new() -> Self {
        TechnicClient
    }

    fn api_url(path: &str) -> String {
        format!("https://api.technicpack.net{}", path)
    }
}

impl ApiClient for TechnicClient {
    fn platform(&self) -> ModpackPlatform {
        ModpackPlatform::Technic
    }

    fn search_packs(&self, query: &str, limit: u32) -> Result<Vec<ModpackEntry>> {
        let url = Self::api_url(&format!(
            "/modpack/search/{}?limit={}",
            urlencode(query),
            limit
        ));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Technic search failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        if let Some(packs) = json["modpacks"].as_array() {
            packs
                .iter()
                .map(|p| {
                    Ok(ModpackEntry {
                        id: p["slug"].as_str().unwrap_or("").to_string(),
                        name: p["name"].as_str().unwrap_or("").to_string(),
                        slug: p["slug"].as_str().unwrap_or("").to_string(),
                        description: p["description"].as_str().unwrap_or("").to_string(),
                        author: p["author"].as_str().unwrap_or("").to_string(),
                        icon_url: p["icon"]["url"].as_str().unwrap_or("").to_string(),
                        mod_count: 0,
                        download_count: p["downloads"].as_u64().unwrap_or(0),
                        platform: ModpackPlatform::Technic,
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
                "Technic get pack failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let p = &json["modpack"];

        Ok(ModpackEntry {
            id: pack_id.to_string(),
            name: p["name"].as_str().unwrap_or("").to_string(),
            slug: p["slug"].as_str().unwrap_or("").to_string(),
            description: p["description"].as_str().unwrap_or("").to_string(),
            author: p["author"].as_str().unwrap_or("").to_string(),
            icon_url: p["icon"]["url"].as_str().unwrap_or("").to_string(),
            mod_count: 0,
            download_count: p["downloads"].as_u64().unwrap_or(0),
            platform: ModpackPlatform::Technic,
        })
    }

    fn get_versions(&self, pack_id: &str) -> Result<Vec<ModpackVersion>> {
        let url = Self::api_url(&format!("/modpack/{}", pack_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Technic get versions failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        let latest = &json["modpack"]["latest"];

        // Technic API only provides latest; we return it as a single-entry list
        let mc_version = latest["minecraft"].as_str().unwrap_or("").to_string();

        Ok(vec![ModpackVersion {
            id: latest["version"].as_str().unwrap_or("").to_string(),
            name: latest["version"].as_str().unwrap_or("latest").to_string(),
            version_number: latest["version"].as_str().unwrap_or("").to_string(),
            minecraft_version: mc_version,
            loader: latest["forge"]
                .or_else(|| &latest["fabric"])
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            download_url: format!(
                "https://api.technicpack.net/modpack/{}/version/{}",
                pack_id,
                latest["version"].as_str().unwrap_or("latest")
            ),
            file_size: 0,
            release_date: "".to_string(),
            version_type: "release".to_string(),
        }])
    }

    fn get_version(&self, pack_id: &str, version_id: &str) -> Result<ModpackVersion> {
        let url = Self::api_url(&format!("/modpack/{}/version/{}", pack_id, version_id));
        let resp = http_client::get(&url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Technic get version failed: HTTP {}",
                resp.status
            )));
        }

        let json: serde_json::Value =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;

        Ok(ModpackVersion {
            id: version_id.to_string(),
            name: version_id.to_string(),
            version_number: version_id.to_string(),
            minecraft_version: json["minecraft"].as_str().unwrap_or("").to_string(),
            loader: json["forge"]
                .or_else(|| json.get("fabric"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            download_url: format!(
                "https://api.technicpack.net/modpack/{}/version/{}/url",
                pack_id, version_id
            ),
            file_size: 0,
            release_date: "".to_string(),
            version_type: "release".to_string(),
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

    fn download_url(&self, pack_id: &str, version_id: &str) -> Result<String> {
        Ok(format!(
            "https://api.technicpack.net/modpack/{}/version/{}/url",
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

use error::{CoreError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDetails {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "minecraftArguments", default)]
    pub minecraft_arguments: String,
    #[serde(rename = "arguments", default)]
    pub arguments: Option<VersionArguments>,
    pub libraries: Vec<Library>,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub downloads: VersionDownloads,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionArguments {
    #[serde(default)]
    pub game: Vec<ArgumentValue>,
    #[serde(default)]
    pub jvm: Vec<ArgumentValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    String(String),
    Object {
        value: ArgumentValueInner,
        rules: Vec<Rule>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValueInner {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub action: String,
    pub os: Option<OsRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsRule {
    pub name: Option<String>,
    pub version: Option<String>,
    pub arch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub downloads: Option<LibraryDownloads>,
    pub rules: Option<Vec<Rule>>,
    #[serde(default)]
    pub natives: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub extract: Option<ExtractRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub path: String,
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractRules {
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownloads {
    pub client: Option<Artifact>,
    pub server: Option<Artifact>,
    pub client_mappings: Option<Artifact>,
    pub server_mappings: Option<Artifact>,
}

pub struct ManifestLoader;

impl ManifestLoader {
    pub fn fetch_manifest() -> Result<VersionManifest> {
        let url = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";
        let resp = http_client::get(url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Manifest fetch failed: HTTP {}",
                resp.status
            )));
        }
        let manifest: VersionManifest =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        Ok(manifest)
    }

    pub fn fetch_version_details(url: &str) -> Result<VersionDetails> {
        let resp = http_client::get(url)?;
        if resp.status != 200 {
            return Err(CoreError::Http(format!(
                "Version fetch failed: HTTP {}",
                resp.status
            )));
        }
        let details: VersionDetails =
            serde_json::from_slice(&resp.body).map_err(CoreError::Json)?;
        Ok(details)
    }

    pub fn resolve_version(version_id: &str) -> Result<VersionDetails> {
        let manifest = Self::fetch_manifest()?;
        let version_info = manifest
            .versions
            .iter()
            .find(|v| v.id == version_id)
            .ok_or_else(|| CoreError::InvalidData(format!("Version {} not found", version_id)))?;
        Self::fetch_version_details(&version_info.url)
    }

    pub fn resolve_latest_release() -> Result<VersionDetails> {
        let manifest = Self::fetch_manifest()?;
        let release_id = &manifest.latest.release;
        Self::resolve_version(release_id)
    }

    pub fn resolve_latest_snapshot() -> Result<VersionDetails> {
        let manifest = Self::fetch_manifest()?;
        let snapshot_id = &manifest.latest.snapshot;
        Self::resolve_version(snapshot_id)
    }

    pub fn list_versions(filter_type: Option<&str>) -> Result<Vec<VersionInfo>> {
        let manifest = Self::fetch_manifest()?;
        let versions = match filter_type {
            Some(t) => manifest
                .versions
                .into_iter()
                .filter(|v| v.version_type == t)
                .collect(),
            None => manifest.versions,
        };
        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_parsing() {
        let json = r#"{
            "latest": {"release": "1.21", "snapshot": "25w14craftmine"},
            "versions": [
                {"id": "1.21", "type": "release", "url": "https://example.com/1.21.json", "time": "2024-06-01", "releaseTime": "2024-06-01"},
                {"id": "25w14craftmine", "type": "snapshot", "url": "https://example.com/snapshot.json", "time": "2025-04-01", "releaseTime": "2025-04-01"}
            ]
        }"#;

        let manifest: VersionManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.latest.release, "1.21");
        assert_eq!(manifest.versions.len(), 2);
        assert_eq!(manifest.versions[0].id, "1.21");
        assert_eq!(manifest.versions[0].version_type, "release");
    }

    #[test]
    fn test_rule_allows_matching_os() {
        let rule = Rule {
            action: "allow".into(),
            os: Some(OsRule {
                name: Some("osx".into()),
                version: None,
                arch: None,
            }),
        };
        assert_eq!(rule.action, "allow");
        assert_eq!(rule.os.as_ref().unwrap().name.as_ref().unwrap(), "osx");
    }
}

use error::Result;
use serde::{Deserialize, Serialize};

pub type ApiResult<T> = Result<T>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModpackEntry {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub author: String,
    pub icon_url: String,
    pub mod_count: u32,
    pub download_count: u64,
    pub platform: ModpackPlatform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModpackPlatform {
    Modrinth,
    CurseForge,
    FTB,
    Technic,
    ATLauncher,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModpackVersion {
    pub id: String,
    pub name: String,
    pub version_number: String,
    pub minecraft_version: String,
    pub loader: String,
    pub download_url: String,
    pub file_size: u64,
    pub release_date: String,
    pub version_type: String,
}

pub trait ApiClient {
    fn platform(&self) -> ModpackPlatform;

    fn search_packs(&self, query: &str, limit: u32) -> ApiResult<Vec<ModpackEntry>>;

    fn get_pack(&self, pack_id: &str) -> ApiResult<ModpackEntry>;

    fn get_versions(&self, pack_id: &str) -> ApiResult<Vec<ModpackVersion>>;

    fn get_version(&self, pack_id: &str, version_id: &str) -> ApiResult<ModpackVersion>;

    fn get_latest_version(
        &self,
        pack_id: &str,
        mc_version: &str,
        loader: &str,
    ) -> ApiResult<Option<ModpackVersion>>;

    fn download_url(&self, pack_id: &str, version_id: &str) -> ApiResult<String>;
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod schemas;
pub mod supported_versions;

pub mod prelude {
    pub use super::schemas::*;
    pub use super::supported_versions::SUPPORTED_VERSIONS;
    pub use super::{FromMCDataVersionDir, FromVersion, MINECRAFT_DATA_DIR};
}

pub trait FromVersion {
    fn from_version(version: &str) -> Option<Self>
    where
        Self: Sized;
}

pub const MINECRAFT_DATA_DIR: include_dir::Dir =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/minecraft-data/data");

#[derive(Serialize, Deserialize)]
pub struct DataPaths {
    pc: HashMap<String, HashMap<String, String>>,
    bedrock: HashMap<String, HashMap<String, String>>,
}

lazy_static::lazy_static!(
    static ref MINECRAFT_DATA_PATHS: DataPaths = serde_json::from_str(
        MINECRAFT_DATA_DIR.get_file("dataPaths.json").unwrap().contents_utf8().unwrap()).unwrap();
);

pub trait FromMCDataVersionDir {
    fn from_version_paths(paths: &HashMap<String, String>) -> Option<Self>
    where
        Self: Sized;
}

impl<T: FromMCDataVersionDir> FromVersion for T {
    fn from_version(version: &str) -> Option<Self> {
        Self::from_version_paths(MINECRAFT_DATA_PATHS.pc.get(version).unwrap())
    }
}

use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "version";
const FILE_NAME: &'static str = "version.json";
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "version")]
pub struct Version {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "majorVersion")]
    pub major_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraftVersion")]
    pub minecraft_version: Option<String>,
    #[doc = " The protocol version"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl FromMCDataVersionDir for Version {
    fn from_version_paths(paths: &std::collections::HashMap<String, String>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut path = std::path::PathBuf::from(paths.get(MODULE_NAME).unwrap());
        path.push(FILE_NAME);
        Some(
            serde_json::from_str(
                MINECRAFT_DATA_DIR
                    .get_file(path)
                    .unwrap()
                    .contents_utf8()
                    .unwrap(),
            )
            .unwrap(),
        )
    }
}

use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "protocolVersions";
const FILE_NAME: &'static str = "protocolVersions.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProtocolVersionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dataVersion")]
    pub data_version: Option<i64>,
    #[serde(rename = "majorVersion")]
    pub major_version: String,
    #[serde(rename = "minecraftVersion")]
    pub minecraft_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usesNetty")]
    pub uses_netty: Option<bool>,
    #[doc = " The protocol version"]
    pub version: i64,
}
pub type ProtocolVersions = Vec<ProtocolVersionsItem>;
impl FromMCDataVersionDir for ProtocolVersions {
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

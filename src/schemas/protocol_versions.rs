use serde::*;
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
    #[serde(rename = "releaseType")]
    pub release_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usesNetty")]
    pub uses_netty: Option<bool>,
    #[doc = " The protocol version"]
    pub version: i64,
}
pub type ProtocolVersions = Vec<ProtocolVersionsItem>;

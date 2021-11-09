use serde::*;
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

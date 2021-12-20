use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "entities";
const FILE_NAME: &'static str = "entities.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntitiesItem {
    #[doc = " The category of an entity : a semantic category"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = " The display name of an entity"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = " The height of the entity"]
    #[serde(default)]
    pub height: Option<f64>,
    #[doc = " The unique identifier for an entity"]
    pub id: i64,
    #[doc = " The internal id of an entity : used in eggs metadata for example"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "internalId")]
    pub internal_id: Option<i64>,
    #[doc = " The name of an entity"]
    pub name: String,
    #[doc = " The type of an entity"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = " The width of the entity"]
    #[serde(default)]
    pub width: Option<f64>,
}
pub type Entities = Vec<EntitiesItem>;
impl FromMCDataVersionDir for Entities {
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

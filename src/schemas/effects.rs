use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "effects";
const FILE_NAME: &'static str = "effects.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EffectsItem {
    #[doc = " The display name of an effect"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = " The unique identifier for an effect"]
    pub id: i64,
    #[doc = " The name of an effect"]
    pub name: String,
    #[doc = " Whether an effect is positive or negative"]
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
pub type Effects = Vec<EffectsItem>;
impl FromMCDataVersionDir for Effects {
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

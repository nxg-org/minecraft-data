use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "mapIcons";
const FILE_NAME: &'static str = "mapIcons.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MapIconsItem {
    #[doc = " Description of the map icon's appearance"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<String>,
    #[doc = " The unique identifier for a map icon"]
    pub id: i64,
    #[doc = " The name of a map icon"]
    pub name: String,
    #[doc = " Visibility in item frames"]
    #[serde(rename = "visibleInItemFrame")]
    pub visible_in_item_frame: bool,
}
pub type MapIcons = Vec<MapIconsItem>;
impl FromMCDataVersionDir for MapIcons {
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

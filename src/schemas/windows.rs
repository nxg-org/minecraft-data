use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "windows";
const FILE_NAME: &'static str = "windows.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WindowsItemItemOpenedWith {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WindowsItemItemSlots {
    #[doc = " The position of the slot or begin of the slot range"]
    pub index: i64,
    #[doc = " The name of the slot or slot range"]
    pub name: String,
    #[doc = " The size of the slot range"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WindowsItem {
    #[doc = " The unique identifier for the window"]
    pub id: String,
    #[doc = " The default displayed name of the window"]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "openedWith")]
    pub opened_with: Option<Vec<WindowsItemItemOpenedWith>>,
    #[doc = " Names of the properties of the window"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    #[doc = " The slots displayed in the window"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<WindowsItemItemSlots>>,
}
pub type Windows = Vec<WindowsItem>;
impl FromMCDataVersionDir for Windows {
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

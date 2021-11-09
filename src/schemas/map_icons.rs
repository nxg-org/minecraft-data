use serde::*;
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

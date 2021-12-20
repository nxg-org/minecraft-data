use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "items";
const FILE_NAME: &'static str = "items.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ItemsItemItemVariations {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub metadata: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ItemsItem {
    #[doc = " The display name of an item"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = " The durability of an item"]
    #[serde(default)]
    pub durability: Option<i64>,
    #[doc = " describes categories of enchants this item can use"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enchantCategories")]
    pub enchant_categories: Option<Vec<String>>,
    #[doc = " describes what items this item can be fixed with in an anvil"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fixedWith")]
    pub fixed_with: Option<Vec<String>>,
    #[doc = " The unique identifier for an item"]
    pub id: i64,
    #[doc = " the amount of durability an item has before being damaged/used"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxDurability")]
    pub max_durability: Option<i64>,
    #[doc = " The name of an item"]
    pub name: String,
    #[doc = " Stack size for an item"]
    #[serde(rename = "stackSize")]
    pub stack_size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<ItemsItemItemVariations>>,
}
pub type Items = Vec<ItemsItem>;
impl FromMCDataVersionDir for Items {
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

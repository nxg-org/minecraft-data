use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "entityLoot";
const FILE_NAME: &'static str = "entityLoot.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntityLootItemItemDrops {
    #[doc = " The percent chance of the item drop to occur"]
    #[serde(rename = "dropChance")]
    pub drop_chance: f64,
    #[doc = " The name of the item being dropped"]
    pub item: String,
    #[doc = " If a player killer is required"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "playerKill")]
    pub player_kill: Option<bool>,
    #[doc = " The min/max of number of items in this item drop stack"]
    #[serde(rename = "stackSizeRange")]
    pub stack_size_range: Vec<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntityLootItem {
    #[doc = " The list of item drops"]
    pub drops: Vec<EntityLootItemItemDrops>,
    #[doc = " The name of the entity"]
    pub entity: String,
}
pub type EntityLoot = Vec<EntityLootItem>;
impl FromMCDataVersionDir for EntityLoot {
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

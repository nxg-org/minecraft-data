use serde::*;
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

use serde::*;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlockLootItemItemDrops {
    #[doc = " The required age of the block for the item drop to occur"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockAge")]
    pub block_age: Option<f64>,
    #[doc = " The percent chance of the item drop to occur"]
    #[serde(rename = "dropChance")]
    pub drop_chance: f64,
    #[doc = " The name of the item being dropped"]
    pub item: String,
    #[doc = " If not having silk touch is required"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "noSilkTouch")]
    pub no_silk_touch: Option<bool>,
    #[doc = " If silk touch is required"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "silkTouch")]
    pub silk_touch: Option<bool>,
    #[doc = " The min/max of number of items in this item drop stack"]
    #[serde(rename = "stackSizeRange")]
    pub stack_size_range: Vec<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlockLootItem {
    #[doc = " The name of the block"]
    pub block: String,
    #[doc = " The list of item drops"]
    pub drops: Vec<BlockLootItemItemDrops>,
}
pub type BlockLoot = Vec<BlockLootItem>;

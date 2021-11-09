use serde::*;
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
    #[doc = " describes categories of enchants this item can use"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enchantCategories")]
    pub enchant_categories: Option<Vec<String>>,
    #[doc = " The unique identifier for an item"]
    pub id: i64,
    #[doc = " the amount of durability an item has before being damaged/used"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxDurability")]
    pub max_durability: Option<i64>,
    #[doc = " The name of an item"]
    pub name: String,
    #[doc = " describes what items this item can be fixed with in an anvil"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "repairWith")]
    pub repair_with: Option<Vec<String>>,
    #[doc = " Stack size for an item"]
    #[serde(rename = "stackSize")]
    pub stack_size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<ItemsItemItemVariations>>,
}
pub type Items = Vec<ItemsItem>;

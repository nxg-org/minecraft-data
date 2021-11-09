use serde::*;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct EnchantmentsItemMaxCost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct EnchantmentsItemMinCost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EnchantmentsItem {
    #[doc = " The category of enchantable items"]
    pub category: String,
    #[doc = " Is a curse, not an enchantment"]
    pub curse: bool,
    #[doc = " Can this enchantment be discovered"]
    pub discoverable: bool,
    #[doc = " The display name of an enchantment"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = " List of enchantment not compatibles"]
    pub exclude: Vec<String>,
    #[doc = " The unique identifier for an enchantment"]
    pub id: i64,
    #[doc = " Max cost equation's coefficients a * level + b"]
    #[serde(rename = "maxCost")]
    pub max_cost: EnchantmentsItemMaxCost,
    #[doc = " The maximum level of an enchantment"]
    #[serde(rename = "maxLevel")]
    pub max_level: i64,
    #[doc = " Min cost equation's coefficients a * level + b"]
    #[serde(rename = "minCost")]
    pub min_cost: EnchantmentsItemMinCost,
    #[doc = " The name of an enchantment"]
    pub name: String,
    #[doc = " Can this enchantment be traded"]
    pub tradeable: bool,
    #[doc = " Can only be found in a treasure, not created"]
    #[serde(rename = "treasureOnly")]
    pub treasure_only: bool,
    #[doc = " Weight of the rarity of the enchantment"]
    pub weight: i64,
}
pub type Enchantments = Vec<EnchantmentsItem>;

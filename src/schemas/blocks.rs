use serde::*;
pub type BlocksItemItemDropsVariant0 = i64;
pub type BlocksItemItemDropsVariant1DropVariant0 = i64;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlocksItemItemDropsVariant1DropVariant1 {
    pub id: i64,
    pub metadata: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlocksItemItemDropsVariant1Drop {
    Variant0(BlocksItemItemDropsVariant1DropVariant0),
    Variant1(BlocksItemItemDropsVariant1DropVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlocksItemItemDropsVariant1 {
    pub drop: BlocksItemItemDropsVariant1Drop,
    #[doc = " maximum number or chance, default : minCount"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxCount")]
    pub max_count: Option<f64>,
    #[doc = " minimum number or chance, default : 1"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minCount")]
    pub min_count: Option<f64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlocksItemItemDrops {
    Variant0(BlocksItemItemDropsVariant0),
    Variant1(BlocksItemItemDropsVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlocksItemItemStates {
    #[doc = " The name of the property"]
    pub name: String,
    #[doc = " The number of possible values"]
    pub num_values: f64,
    #[doc = " The type of the property"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = " The possible values of the property"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<serde_json::Value>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlocksItemItemVariations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub metadata: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlocksItem {
    #[doc = " BoundingBox of a block"]
    #[serde(rename = "boundingBox")]
    pub bounding_box: serde_json::Value,
    #[doc = " Default state id"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultState")]
    pub default_state: Option<i64>,
    #[doc = " true if a block is diggable"]
    pub diggable: bool,
    #[doc = " The display name of a block"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub drops: Vec<BlocksItemItemDrops>,
    #[doc = " Light emitted by that block"]
    #[serde(rename = "emitLight")]
    pub emit_light: i64,
    #[doc = " Light filtered by that block"]
    #[serde(rename = "filterLight")]
    pub filter_light: i64,
    #[doc = " Hardness of a block"]
    #[serde(default)]
    pub hardness: Option<f64>,
    #[doc = " Using one of these tools is required to harvest a block, without that you get a 3.33x time "]
    #[doc = " penalty."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "harvestTools")]
    pub harvest_tools: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " The unique identifier for a block"]
    pub id: i64,
    #[doc = " Material of a block"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[doc = " Maximum state id"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxStateId")]
    pub max_state_id: Option<i64>,
    #[doc = " Minimum state id"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minStateId")]
    pub min_state_id: Option<i64>,
    #[doc = " The name of a block"]
    pub name: String,
    #[doc = " Blast resistance"]
    #[serde(default)]
    pub resistance: Option<f64>,
    #[doc = " Stack size for a block"]
    #[serde(rename = "stackSize")]
    pub stack_size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<BlocksItemItemStates>>,
    #[doc = " true if a block is transparent"]
    pub transparent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<BlocksItemItemVariations>>,
}
pub type Blocks = Vec<BlocksItem>;

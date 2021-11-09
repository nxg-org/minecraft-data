use serde::*;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsConstantItemData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsConstant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TintsConstantItemData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsFoliageItemData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsFoliage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TintsFoliageItemData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsGrassItemData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsGrass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TintsGrassItemData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsRedstoneItemData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<i64>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsRedstone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TintsRedstoneItemData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsWaterItemData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct TintsWater {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TintsWaterItemData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "tints")]
pub struct Tints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TintsConstant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliage: Option<TintsFoliage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass: Option<TintsGrass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redstone: Option<TintsRedstone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water: Option<TintsWater>,
}

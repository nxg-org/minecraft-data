use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "tints";
const FILE_NAME: &'static str = "tints.json";
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
impl FromMCDataVersionDir for Tints {
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

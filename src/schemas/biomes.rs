use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "biomes";
const FILE_NAME: &'static str = "biomes.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BiomesItem {
    #[doc = " The category of a biome"]
    pub category: String,
    #[doc = " The color in a biome"]
    pub color: i64,
    #[doc = " The depth of a biome"]
    pub depth: f64,
    #[doc = " The dimension of a biome: overworld, nether or end"]
    pub dimension: String,
    #[doc = " The display name of a biome"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = " The unique identifier for a biome"]
    pub id: i64,
    #[doc = " The name of a biome"]
    pub name: String,
    #[doc = " The type of precipitation: none, rain or snow"]
    pub precipitation: String,
    #[doc = " How much rain there is in a biome"]
    pub rainfall: f64,
    #[doc = " An indicator for the temperature in a biome"]
    pub temperature: f64,
}
pub type Biomes = Vec<BiomesItem>;
impl FromMCDataVersionDir for Biomes {
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

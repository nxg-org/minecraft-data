use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "blockCollisionShapes";
const FILE_NAME: &'static str = "blockCollisionShapes.json";
pub type BlockCollisionShapesBlocksVariant0 = f64;
pub type BlockCollisionShapesBlocksVariant1 = Vec<f64>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockCollisionShapesBlocks {
    Variant0(BlockCollisionShapesBlocksVariant0),
    Variant1(BlockCollisionShapesBlocksVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "block_collision_shapes")]
pub struct BlockCollisionShapes {
    #[doc = " Each block's collision shape id(s)."]
    pub blocks: ::std::collections::BTreeMap<String, BlockCollisionShapesBlocks>,
    #[doc = " Collision shapes by id, each shape being composed of a list of collision boxes."]
    pub shapes: ::std::collections::BTreeMap<String, Vec<Vec<f64>>>,
}
impl FromMCDataVersionDir for BlockCollisionShapes {
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

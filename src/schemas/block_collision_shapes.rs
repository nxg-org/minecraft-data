use serde::*;
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

use serde::*;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FoodsItemItemVariations {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub metadata: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FoodsItem {
    #[doc = " The display name of an item"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = " The effective quality of the food item"]
    #[serde(rename = "effectiveQuality")]
    pub effective_quality: f64,
    #[doc = " The amount of food points the food item replenishes"]
    #[serde(rename = "foodPoints")]
    pub food_points: f64,
    #[doc = " The unique identifier for an item"]
    pub id: i64,
    #[doc = " The name of an item"]
    pub name: String,
    #[doc = " The amount of saturation points the food  restores"]
    pub saturation: f64,
    #[doc = " The saturation ratio of the food item"]
    #[serde(rename = "saturationRatio")]
    pub saturation_ratio: f64,
    #[doc = " Stack size for an item"]
    #[serde(rename = "stackSize")]
    pub stack_size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<FoodsItemItemVariations>>,
}
pub type Foods = Vec<FoodsItem>;

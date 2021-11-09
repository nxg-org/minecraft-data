use serde::*;
pub type Count = i64;
#[doc = " A single numerical ID or null."]
pub type Id = Option<i64>;
#[doc = " A list of id and metadata. This is preferred if there are many items at once, e.g. in a shape."]
pub type IdMetadataArray = Vec<Id>;
#[doc = " A dictionary of at least id, optionally metadata and count. This is preferred if there are not "]
#[doc = " many items at once, e.g. result in a recipe."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "id_metadata_count_object")]
pub struct IdMetadataCountObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<Count>,
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
pub type Ingredients = Vec<RecipeItem>;
pub type Metadata = i64;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RecipeMetadata {
    Variant0(ShapedRecipe),
    Variant1(ShapelessRecipe),
}
pub type Recipe = RecipeMetadata;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RecipeItemMetadata {
    Variant0(Id),
    Variant1(IdMetadataArray),
    Variant2(IdMetadataCountObject),
}
#[doc = " An item can be represented different ways."]
pub type RecipeItem = RecipeItemMetadata;
#[doc = " A shape is a list of rows, which are lists of items. There must be at least one row with at "]
#[doc = " least one item in it. All rows must have the same length. Empty rows at the beginning or end of "]
#[doc = " a shape may be omitted. Empty colums at the end may also be omitted. When an item can be "]
#[doc = " crafted in a 2x2 grid, the shape may not be larger than 2x2."]
pub type Shape = Vec<ShapeRow>;
pub type ShapeRow = Vec<RecipeItem>;
#[doc = " A shaped recipe is a dictionary of result, inShape and optionally outShape"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "shaped_recipe")]
pub struct ShapedRecipe {
    #[serde(rename = "inShape")]
    pub in_shape: Shape,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outShape")]
    pub out_shape: Option<Shape>,
    pub result: RecipeItem,
}
#[doc = " A shapeless recipe is a dictionary of result and ingredients"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "shapeless_recipe")]
pub struct ShapelessRecipe {
    pub ingredients: Ingredients,
    pub result: RecipeItem,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "recipes")]
pub struct Recipes {}

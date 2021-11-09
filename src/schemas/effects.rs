use serde::*;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EffectsItem {
    #[doc = " The display name of an effect"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = " The unique identifier for an effect"]
    pub id: i64,
    #[doc = " The name of an effect"]
    pub name: String,
    #[doc = " Whether an effect is positive or negative"]
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
pub type Effects = Vec<EffectsItem>;

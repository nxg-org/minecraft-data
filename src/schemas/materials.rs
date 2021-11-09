use serde::*;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "materials")]
pub struct Materials {}

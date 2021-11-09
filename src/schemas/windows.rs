use serde::*;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WindowsItemItemOpenedWith {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WindowsItemItemSlots {
    #[doc = " The position of the slot or begin of the slot range"]
    pub index: i64,
    #[doc = " The name of the slot or slot range"]
    pub name: String,
    #[doc = " The size of the slot range"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WindowsItem {
    #[doc = " The unique identifier for the window"]
    pub id: String,
    #[doc = " The default displayed name of the window"]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "openedWith")]
    pub opened_with: Option<Vec<WindowsItemItemOpenedWith>>,
    #[doc = " Names of the properties of the window"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    #[doc = " The slots displayed in the window"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<WindowsItemItemSlots>>,
}
pub type Windows = Vec<WindowsItem>;

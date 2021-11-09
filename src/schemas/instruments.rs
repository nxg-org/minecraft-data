use serde::*;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InstrumentsItem {
    #[doc = " The unique identifier for an instrument"]
    pub id: i64,
    #[doc = " The name of an instrument"]
    pub name: String,
    #[doc = " The sound ID played by this instrument"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
}
pub type Instruments = Vec<InstrumentsItem>;

use serde::*;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ParticlesItem {
    #[doc = " The unique identifier for a particle"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = " The name of a particle"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type Particles = Vec<ParticlesItem>;

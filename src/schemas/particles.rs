use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "particles";
const FILE_NAME: &'static str = "particles.json";
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
impl FromMCDataVersionDir for Particles {
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

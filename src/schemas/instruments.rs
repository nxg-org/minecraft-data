use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "instruments";
const FILE_NAME: &'static str = "instruments.json";
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InstrumentsItem {
    #[doc = " The unique identifier for an instrument"]
    pub id: i64,
    #[doc = " The name of an instrument"]
    pub name: String,
}
pub type Instruments = Vec<InstrumentsItem>;
impl FromMCDataVersionDir for Instruments {
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

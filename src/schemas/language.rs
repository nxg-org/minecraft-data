use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
use serde::*;
const MODULE_NAME: &'static str = "language";
const FILE_NAME: &'static str = "language.json";
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "language")]
pub struct Language {}
impl FromMCDataVersionDir for Language {
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

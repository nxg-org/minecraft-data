use convert_case::Casing;
use schemafy_lib;
use std::{io::Write, path::PathBuf, process::Command};

const MINECRAFT_DATA_SCHEMA_PATH: &'static str = "./minecraft-data/schemas";
const SCHEMA_OUT_FILES: &'static str = "./src/schemas";

fn main() {
    iter_schema_dir(MINECRAFT_DATA_SCHEMA_PATH);
    Command::new("cargo").arg("fmt").spawn().unwrap();
}

fn iter_schema_dir<T: Into<PathBuf> + Copy>(dir_path: T) {
    let dir_path: PathBuf = dir_path.into();
    let mut out_folder_path: PathBuf = SCHEMA_OUT_FILES.into();
    out_folder_path.push(dir_path.strip_prefix(MINECRAFT_DATA_SCHEMA_PATH).unwrap());
    std::fs::create_dir_all(&out_folder_path).unwrap();
    let mut mod_file = std::fs::File::create(out_folder_path.join("mod.rs")).unwrap();
    for entry in std::fs::read_dir(dir_path).unwrap().into_iter() {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                let schema_stem = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("_schema", "")
                    .from_case(convert_case::Case::Camel)
                    .to_case(convert_case::Case::Snake);
                if file_type.is_file() {
                    let mut out = schemafy_lib::GeneratorBuilder::default()
                        .with_input_file(&entry.path())
                        .with_root_name_str(&schema_stem)
                        .build()
                        .generate()
                        .to_string();
                    if out.contains("Serialize") || out.contains("Deserialize") {
                        out = format!("use serde::*;\n{}", out);
                    }
                    std::fs::write(out_folder_path.join(&schema_stem).with_extension("rs"), out)
                        .unwrap();
                    mod_file
                        .write_all(format!("pub mod {};\n", schema_stem).as_bytes())
                        .unwrap();
                }
                if file_type.is_dir() {
                    iter_schema_dir(entry.path().to_str().unwrap());
                }
            }
        }
    }
}

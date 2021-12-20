#![feature(extend_one)]
use convert_case::{Case, Casing};
use proc_macro2::Span;
use schemafy_lib;
use std::{io::Write, path::PathBuf, process::Command};

const MINECRAFT_DATA_SCHEMA_PATH: &'static str = "./minecraft-data/schemas";
const SCHEMA_OUT_FILES: &'static str = "./src/schemas";

fn main() {
    if let Err(_) = std::fs::read_dir(SCHEMA_OUT_FILES) {
        iter_schema_dir(MINECRAFT_DATA_SCHEMA_PATH);
    }
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
                let schema_name = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("_schema", "")
                    .from_case(convert_case::Case::Camel);
                let schema_stem = schema_name.to_case(convert_case::Case::Snake);
                if file_type.is_file() {
                    let def_tokens = schemafy_lib::GeneratorBuilder::default()
                        .with_input_file(&entry.path())
                        .with_root_name_str(&schema_stem)
                        .build()
                        .generate();
                    let mut tokenstream = proc_macro2::TokenStream::new();
                    for token in def_tokens.clone() {
                        let str_token = token.to_string();
                        if str_token.contains("Serialize") || str_token.contains("Deserialize") {
                            tokenstream.extend(quote::quote! {use serde::*;});
                            break;
                        }
                    }
                    tokenstream.extend(quote::quote! {
                        use crate::{FromMCDataVersionDir, MINECRAFT_DATA_DIR};
                        const MODULE_NAME: &'static str =
                    });
                    tokenstream.extend_one(proc_macro2::TokenTree::Literal(
                        proc_macro2::Literal::string(&schema_name.to_case(Case::Camel)),
                    ));
                    tokenstream.extend(quote::quote! {;const FILE_NAME: &'static str =});
                    tokenstream.extend_one(proc_macro2::TokenTree::Literal(
                        proc_macro2::Literal::string(&{
                            let mut a = schema_name.to_case(Case::Camel);
                            a.push_str(".json");
                            a
                        }),
                    ));
                    tokenstream.extend_one(quote::quote! {;});
                    tokenstream.extend(def_tokens);
                    tokenstream.extend(quote::quote! {impl FromMCDataVersionDir for});
                    tokenstream.extend_one(proc_macro2::TokenTree::Ident(proc_macro2::Ident::new(
                        &schema_name.to_case(Case::UpperCamel),
                        Span::call_site(),
                    )));
                    tokenstream.extend(quote::quote! { {
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
                    }});
                    let out = tokenstream.to_string();
                    std::fs::write(out_folder_path.join(&schema_stem).with_extension("rs"), out)
                        .unwrap();
                    mod_file
                        .write_all(
                            format!(
                                "pub mod {};pub use {}::{};",
                                schema_stem,
                                schema_stem,
                                schema_name.to_case(Case::UpperCamel)
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                }
                if file_type.is_dir() {
                    // no useful typings are generated for protocol_types anyways, so why bother?
                    // iter_schema_dir(entry.path().to_str().unwrap());
                }
            }
        }
    }
}

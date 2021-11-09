use serde::*;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ArgumentNodeItemChildren {
    Variant0(LiteralNode),
    Variant1(ArgumentNode),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ArgumentNodeParser {
    #[serde(default)]
    pub modifier: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ArgumentNode {
    pub children: Vec<ArgumentNodeItemChildren>,
    pub executable: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser: Option<ArgumentNodeParser>,
    pub redirects: Vec<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LiteralNodeItemChildren {
    Variant0(LiteralNode),
    Variant1(ArgumentNode),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LiteralNode {
    pub children: Vec<LiteralNodeItemChildren>,
    pub executable: bool,
    pub name: String,
    pub redirects: Vec<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ParserInfo {
    pub examples: Vec<String>,
    #[serde(default)]
    pub modifier: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub parser: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RootNodeItemChildren {
    Variant0(LiteralNode),
    Variant1(ArgumentNode),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RootNode {
    pub children: Vec<RootNodeItemChildren>,
    pub executable: bool,
    pub name: String,
    pub redirects: Vec<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "commands")]
pub struct Commands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<RootNode>,
    pub parsers: Vec<ParserInfo>,
}

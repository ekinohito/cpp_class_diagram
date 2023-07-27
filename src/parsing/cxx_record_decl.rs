use serde::Deserialize;

use super::{Base, CXXType};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CXXRecordDeclBase {
    pub access: String,
    pub written_access: String,
    #[serde(rename = "type")]
    pub cxx_type: CXXType,
}

#[derive(Deserialize, Debug, Clone)]
/// Represents a C++ struct/union/class.
pub struct CXXRecordDecl {
    #[serde(flatten)]
    pub base: Base,

    #[serde(default)]
    pub name: String,
    #[serde(rename = "tagUsed")]
    pub tag_used: String,
    #[serde(default)]
    pub bases: Vec<CXXRecordDeclBase>,
}

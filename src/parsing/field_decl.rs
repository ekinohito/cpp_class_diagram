use serde::Deserialize;

use super::{CXXType, Base};

#[derive(Deserialize, Debug, Clone)]
/// Represents a member of a struct/union/class.
pub struct FieldDecl{
    #[serde(flatten)]
    pub base: Base,

    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    pub cxx_type: CXXType,
}
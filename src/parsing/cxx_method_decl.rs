use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// Represents a static or instance method of a struct/union/class.
pub struct CXXMethodDecl {
    #[serde(flatten)]
    pub base: Base,

    pub name: String,
}

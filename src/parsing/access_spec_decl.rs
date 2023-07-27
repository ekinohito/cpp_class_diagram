use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// Represents an access specifier followed by colon ':'.
pub struct AccessSpecDecl{
    #[serde(flatten)]
    pub base: Base,

    pub access: String,
}
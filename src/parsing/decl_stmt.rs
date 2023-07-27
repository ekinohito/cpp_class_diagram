use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// Adaptor class for mixing declarations with statements and expressions.
pub struct DeclStmt{
    #[serde(flatten)]
    pub base: Base,
}
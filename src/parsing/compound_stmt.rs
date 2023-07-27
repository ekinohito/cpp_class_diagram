use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents a group of statements like { stmt stmt }.
pub struct CompoundStmt{
    #[serde(flatten)]
    pub base: Base,
}
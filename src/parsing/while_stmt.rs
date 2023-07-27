use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents a 'while' stmt.
pub struct WhileStmt{
    #[serde(flatten)]
    pub base: Base,
}
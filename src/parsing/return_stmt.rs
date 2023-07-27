use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents a return, optionally of an expression: return; return 4;.
pub struct ReturnStmt {
    #[serde(flatten)]
    pub base: Base,
}

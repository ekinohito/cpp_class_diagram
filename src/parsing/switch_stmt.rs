use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents a 'switch' stmt.
pub struct SwitchStmt {
    #[serde(flatten)]
    pub base: Base,
}

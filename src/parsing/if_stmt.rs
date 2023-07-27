use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents an if/then/else.
pub struct IfStmt {
    #[serde(flatten)]
    pub base: Base,

}

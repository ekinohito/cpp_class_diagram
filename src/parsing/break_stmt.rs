use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents a break. 
pub struct BreakStmt{
    #[serde(flatten)]
    pub base: Base,
}
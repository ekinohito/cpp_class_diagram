use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents a 'for (init;cond;inc)' stmt. 
pub struct ForStmt{
    #[serde(flatten)]
    pub base: Base,
}
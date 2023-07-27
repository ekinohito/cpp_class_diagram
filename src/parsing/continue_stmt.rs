use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents a continue. 
pub struct ContinueStmt{
    #[serde(flatten)]
    pub base: Base,
}
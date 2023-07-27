use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// Represent a case statement. 
pub struct CaseStmt{
    #[serde(flatten)]
    pub base: Base,
}
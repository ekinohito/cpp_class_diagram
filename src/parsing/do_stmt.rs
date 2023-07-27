use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// This represents a 'do/while' stmt. 
pub struct DoStmt{
    #[serde(flatten)]
    pub base: Base,
}
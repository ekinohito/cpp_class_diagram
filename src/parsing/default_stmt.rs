use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// Represent a default case statement. 
pub struct DefaultStmt{
    #[serde(flatten)]
    pub base: Base,
}
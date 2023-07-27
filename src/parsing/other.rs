use serde::Deserialize;

use super::Base;

#[derive(Deserialize, Debug, Clone)]
/// Catch-all class for useless nodes
pub struct Other{
    #[serde(flatten)]
    pub base: Base,
}
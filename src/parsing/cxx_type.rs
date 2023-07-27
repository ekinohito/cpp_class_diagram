use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CXXType {
    #[serde(rename = "qualType")]
    pub qual_type: String,
}
use clang_ast::{SourceLocation, SourceRange};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// loc and range
pub struct Base {
    pub loc: Option<SourceLocation>,
    pub range: Option<SourceRange>,
}
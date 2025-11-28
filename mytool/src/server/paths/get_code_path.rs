use super::super::prelude::*;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCodePath {
    owner: String,
    repo: String,
    #[serde(rename = "path")]
    file_path: String,
}
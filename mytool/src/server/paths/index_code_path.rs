use super::super::prelude::*;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexCodePath {
    owner: String,
    repo: String,
    #[serde(rename = "path")]
    file_path: String,
}
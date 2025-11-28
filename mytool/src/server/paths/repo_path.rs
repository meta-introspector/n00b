use super::super::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoPath {
    owner: String,
    repo: String,
}
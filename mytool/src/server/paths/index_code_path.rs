use super::super::prelude::*;
use serde::Deserialize;

/// Represents the path parameters for the code indexing endpoint.
///
/// This struct is used by Actix-web to deserialize path segments for
/// routes that initiate the indexing of a specific code file within a repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexCodePath {
    /// The owner (user or organization) of the GitHub repository.
    pub owner: String,
    /// The name of the GitHub repository.
    pub repo: String,
    /// The path to the file within the repository, extracted from the wildcard segment.
    #[serde(rename = "path")]
    pub file_path: String,
}
use super::super::prelude::*;
use serde::Deserialize;

/// Represents the path parameters for retrieving indexed code content.
///
/// This struct is used by Actix-web to deserialize path segments for
/// routes that fetch the content of an already indexed code file.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCodePath {
    /// The owner (user or organization) of the GitHub repository where the code originated.
    pub owner: String,
    /// The name of the GitHub repository where the code originated.
    pub repo: String,
    /// The path to the file within the repository, extracted from the wildcard segment.
    #[serde(rename = "path")]
    pub file_path: String,
}
use super::super::prelude::*;

/// Represents the path parameters for a GitHub repository endpoint.
///
/// This struct is used by Actix-web to deserialize path segments into
/// a structured format, specifically for routes dealing with repository information.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoPath {
    /// The owner (user or organization) of the GitHub repository.
    pub owner: String,
    /// The name of the GitHub repository.
    pub repo: String,
}
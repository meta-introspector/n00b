use super::super::prelude::*;

/// Represents the path parameters for a GitHub organization's repositories endpoint.
///
/// This struct is used by Actix-web to deserialize path segments into
/// a structured format, specifically for routes dealing with an organization's repositories.
#[derive(Debug, Serialize, Deserialize)]
pub struct OrgReposPath {
    /// The GitHub organization login extracted from the URL path.
    pub org: String,
}
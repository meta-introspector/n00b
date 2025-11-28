use super::super::prelude::*;

/// Represents the path parameters for a GitHub user endpoint.
///
/// This struct is used by Actix-web to deserialize path segments into
/// a structured format, specifically for routes dealing with user information.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPath {
    /// The GitHub username extracted from the URL path.
    pub username: String,
}
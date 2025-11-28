use super::super::prelude::*;

/// Represents the path parameters for endpoints listing a user's repositories.
///
/// This struct is used by Actix-web to deserialize path segments for
/// routes dealing with a specific user's public, starred, or forked repositories.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserReposPath {
    /// The GitHub username extracted from the URL path.
    pub username: String,
}
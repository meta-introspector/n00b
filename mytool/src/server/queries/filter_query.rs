use super::super::prelude::*;

/// Represents an optional filter query parameter for API endpoints.
///
/// This struct is used by Actix-web to deserialize URL query parameters,
/// allowing clients to filter results based on a provided string.
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterQuery {
    /// The optional query string used to filter results.
    pub query: Option<String>,
}
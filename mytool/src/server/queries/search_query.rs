use super::super::prelude::*;

/// Represents the query parameter for a search API endpoint.
///
/// This struct is used by Actix-web to deserialize URL query parameters,
/// specifically for routes that perform a search operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The search query string.
    pub q: String,
}
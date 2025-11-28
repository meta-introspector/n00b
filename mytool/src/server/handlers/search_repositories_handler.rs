use super::super::prelude::*;
use super::super::state::AppState;
use super::super::queries::search_query::SearchQuery; // Explicit module import for query struct
use actix_web::web; // Explicitly import web
use log::error;

/// Handles requests to search for GitHub repositories.
///
/// This endpoint allows clients to search for GitHub repositories using a
/// query string. It leverages the `CachedGitHubClient` for efficient
/// searching with caching capabilities.
///
/// # Arguments
/// * `query` - A `web::Query` extractor containing the `SearchQuery` struct,
///             which deserializes the `q` (query) parameter from the URL.
/// * `data` - A `web::Data` extractor providing access to the shared `AppState`,
///            including the `CachedGitHubClient`.
///
/// # Returns
/// A `Result` which resolves to an `HttpResponse`.
/// - On success, returns `HttpResponse::Ok()` with a JSON array of `RepoMetadata`
///   matching the search query.
/// - On failure, returns `HttpResponse::InternalServerError()` with an error message.
pub async fn search_repositories_handler(
    query: web::Query<SearchQuery>,
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let q = &query.q;
    match data.github_client.search_repositories(q).await {
        Ok(results) => Ok(HttpResponse::Ok().json(results.items)), // Return only items for simplicity
        Err(e) => {
            log::error!("Error searching repositories with query '{}': {:?}", q, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to search repositories: {}", e)))
        }
    }
}

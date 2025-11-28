use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::repo_path::RepoPath; // Explicit module import for path struct
use actix_web::web; // Explicitly import web
use log::error;

/// Handles requests to retrieve GitHub repository information.
///
/// This endpoint fetches public details for a specified GitHub repository
/// (owner and repository name). It uses the `CachedGitHubClient` from the
/// application state for efficient data retrieval with caching.
///
/// # Arguments
/// * `path` - A `web::Path` extractor containing the `RepoPath` struct, which
///            deserializes the `{owner}` and `{repo}` from the URL path.
/// * `data` - A `web::Data` extractor providing access to the shared `AppState`,
///            including the `CachedGitHubClient`.
///
/// # Returns
/// A `Result` which resolves to an `HttpResponse`.
/// - On success, returns `HttpResponse::Ok()` with the `RepoMetadata` as JSON.
/// - On failure (e.g., repository not found, API error), returns
///   `HttpResponse::InternalServerError()` with an error message.
pub async fn get_repo(path: web::Path<RepoPath>, data: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    let owner = path.owner.as_str();
    let repo_name = path.repo.as_str();
    match data.github_client.get_repo_info(owner, repo_name).await {
        Ok(repo_info) => Ok(HttpResponse::Ok().json(repo_info)),
        Err(e) => {
            log::error!("Error fetching repo {}/{}: {:?}", owner, repo_name, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to fetch repo {}/{}: {}", owner, repo_name, e)))
        }
    }
}

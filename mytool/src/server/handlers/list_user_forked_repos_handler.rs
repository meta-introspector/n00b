use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::UserReposPath;

/// Handles requests to list repositories forked by a specific GitHub user.
///
/// This endpoint fetches a list of repositories that a given GitHub user has forked.
/// It uses the `CachedGitHubClient` from the application state for efficient
/// data retrieval with caching.
///
/// # Arguments
/// * `path` - A `web::Path` extractor containing the `UserReposPath` struct, which
///            deserializes the `{username}` from the URL path.
/// * `data` - A `web::Data` extractor providing access to the shared `AppState`,
///            including the `CachedGitHubClient`.
///
/// # Returns
/// A `Result` which resolves to an `HttpResponse`.
/// - On success, returns `HttpResponse::Ok()` with a JSON array of `RepoMetadata`
///   representing the forked repositories.
/// - On failure, returns `HttpResponse::InternalServerError()` with an error message.
pub async fn list_user_forked_repos_handler(
    path: web::Path<UserReposPath>,
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let username = path.username.as_str();
    match data.github_client.list_user_forked_repos(username).await {
        Ok(repos) => Ok(HttpResponse::Ok().json(repos)),
        Err(e) => {
            log::error!("Error listing forked repositories for user '{}': {:?}", username, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to list forked repositories: {}", username)))
        }
    }
}

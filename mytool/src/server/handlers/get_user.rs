use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::user_path::UserPath; // Explicit module import for path struct
use actix_web::web; // Explicitly import web
use log::error;

/// Handles requests to retrieve GitHub user information.
///
/// This endpoint fetches public profile information for a specified GitHub user.
/// It utilizes the `CachedGitHubClient` from the application state to efficiently
/// retrieve user data, leveraging caching to reduce direct GitHub API calls.
///
/// # Arguments
/// * `path` - A `web::Path` extractor containing the `UserPath` struct, which
///            deserializes the `{username}` from the URL path.
/// * `data` - A `web::Data` extractor providing access to the shared `AppState`,
///            including the `CachedGitHubClient` and `CodeIndexer`.
///
/// # Returns
/// A `Result` which resolves to an `HttpResponse`.
/// - On success, returns `HttpResponse::Ok()` with the `UserMetadata` as JSON.
/// - On failure (e.g., user not found, API error), returns `HttpResponse::InternalServerError()`
///   with an error message. In a production system, different HTTP status codes
///   (e.g., `404 Not Found`) might be returned for specific error types.
pub async fn get_user(path: web::Path<UserPath>, data: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    let username = path.username.as_str();
    match data.github_client.get_user_info(username).await {
        Ok(user_info) => Ok(HttpResponse::Ok().json(user_info)),
        Err(e) => {
            log::error!("Error fetching user {}: {:?}", username, e);
            // In a real application, you might want to return different status codes based on the error type
            // e.g., 404 for not found, 500 for internal server error
            Ok(HttpResponse::InternalServerError().body(format!("Failed to fetch user {}: {}", username, e)))
        }
    }
}

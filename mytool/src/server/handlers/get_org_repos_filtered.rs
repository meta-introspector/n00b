use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::org_repos_path::OrgReposPath; // Explicit module import for path struct
use super::super::queries::filter_query::FilterQuery; // Explicit module import for query struct
use crate::github::filter_repos; // filter_repos is not in prelude
use actix_web::web; // Explicitly import web
use log::error;

/// Handles requests to retrieve and filter GitHub organization repositories.
///
/// This endpoint fetches a list of public repositories for a specified GitHub
/// organization. It supports an optional query parameter to filter these
/// repositories by keywords present in their name, description, topics, or language.
///
/// # Arguments
/// * `path` - A `web::Path` extractor containing the `OrgReposPath` struct, which
///            deserializes the `{org}` from the URL path.
/// * `query` - A `web::Query` extractor containing the `FilterQuery` struct,
///             which deserializes the optional `query` parameter from the URL.
/// * `data` - A `web::Data` extractor providing access to the shared `AppState`,
///            including the `CachedGitHubClient`.
///
/// # Returns
/// A `Result` which resolves to an `HttpResponse`.
/// - On success, returns `HttpResponse::Ok()` with a JSON array of `RepoMetadata`.
/// - On failure, returns `HttpResponse::InternalServerError()` with an error message.
pub async fn get_org_repos_filtered(
    path: web::Path<OrgReposPath>,
    query: web::Query<FilterQuery>,
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let org = path.org.as_str();
    match data.github_client.list_org_repos(org).await {
        Ok(repos) => {
            if let Some(q) = &query.query {
                let filters: Vec<&str> = q.split_whitespace().collect();
                let filtered_repos = filter_repos(repos, &filters);
                Ok(HttpResponse::Ok().json(filtered_repos))
            } else {
                Ok(HttpResponse::Ok().json(repos))
            }
        }
        Err(e) => {
            log::error!("Error fetching org {} repositories: {:?}", org, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to fetch org {} repositories: {}", org, e)))
        }
    }
}

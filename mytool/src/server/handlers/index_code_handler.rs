use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::IndexCodePath;

/// Handles requests to index a specific code file from a GitHub repository.
///
/// This endpoint fetches the content of a specified file from GitHub, extracts
/// predefined keywords, and then stores both the file content and its metadata
/// in the `CodeIndexer`. This is a core operation for the Meta Introapector feature
/// of the MCP Tycoon Game.
///
/// # Arguments
/// * `path` - A `web::Path` extractor containing the `IndexCodePath` struct, which
///            deserializes the `{owner}`, `{repo}`, and `{path}` from the URL path.
/// * `data` - A `web::Data` extractor providing access to the shared `AppState`,
///            including the `CachedGitHubClient` and `CodeIndexer`.
///
/// # Returns
/// A `Result` which resolves to an `HttpResponse`.
/// - On successful fetching and indexing, returns `HttpResponse::Ok()` with the
///   `CodeFileMetadata` of the indexed file as JSON.
/// - On failure (e.g., file not found on GitHub, API error, indexing error), returns
///   `HttpResponse::InternalServerError()` with an error message.
pub async fn index_code_handler(
    path: web::Path<IndexCodePath>,
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let owner = path.owner.as_str();
    let repo = path.repo.as_str();
    let file_path = path.file_path.as_str();

    // Fetch content from GitHub
    let file_content = match data.github_client.get_repo_content(owner, repo, file_path).await {
        Ok(content) => content,
        Err(e) => {
            log::error!("Error fetching content for {}/{}/{}: {:?}", owner, repo, file_path, e);
            return Ok(HttpResponse::InternalServerError().body(format!("Failed to fetch code content: {}", e)));
        }
    };

    // Define keywords to search for
    let keywords = vec![
        "server".to_string(), "actix-web".to_string(), "warp".to_string(),
        "tokio::main".to_string(), "mcp".to_string(), "Router".to_string(),
        "Handler".to_string(), "Service".to_string(), "App".to_string(), "HttpServer".to_string(),
    ];
    let found_keywords: Vec<String> = keywords.into_iter()
        .filter(|k| file_content.contains(k))
        .collect();

    // Index content
    match data.code_indexer.index_code_file(owner, repo, file_path, &file_content, found_keywords) {
        Ok(metadata) => Ok(HttpResponse::Ok().json(metadata)),
        Err(e) => {
            log::error!("Error indexing code file {}/{}/{}: {:?}", owner, repo, file_path, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to index code file: {}", e)))
        }
    }
}

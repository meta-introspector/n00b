use super::prelude::*;
use super::state::AppState;
use super::paths::{UserPath, RepoPath, OrgReposPath, IndexCodePath, GetCodePath, UserReposPath}; // Corrected: removed FilterQuery, SearchQuery from here
use super::queries::{FilterQuery, SearchQuery};
use super::handlers::{
    get_user, get_repo, get_org_repos_filtered, search_repositories_handler,
    list_starred_repos_handler, list_user_forked_repos_handler, index_code_handler,
    get_indexed_code_handler, list_indexed_code_metadata_handler
};

/// Initializes and runs the Actix-web HTTP server for the `mytool` application.
///
/// This function sets up the application state with the provided GitHub client
/// and code indexer, configures the Actix-web application with various API routes,
/// and binds the server to the specified listen address.
///
/// The server exposes endpoints for interacting with the GitHub API (via a caching
/// client) and for indexing/retrieving code files.
///
/// # Arguments
/// * `github_client` - An `Arc` to a `CachedGitHubClient` instance for GitHub API access.
/// * `code_indexer` - An `Arc` to a `CodeIndexer` instance for managing indexed code.
/// * `listen_address` - A `String` specifying the address and port for the server to listen on (e.g., "127.0.0.1:8080").
///
/// # Returns
/// An `anyhow::Result` indicating success or failure. The server runs asynchronously
/// until an error occurs or it is explicitly shut down.
pub async fn run_server(
    github_client: Arc<CachedGitHubClient>,
    code_indexer: Arc<CodeIndexer>,
    listen_address: String,
) -> anyhow::Result<()> {
    let app_state = web::Data::new(AppState { 
        github_client,
        code_indexer,
    });

    log::info!("Starting Actix-web server on: {}", listen_address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/api/github")
                    .route("/users/{username}", web::get().to(get_user))
                    .route("/repos/{owner}/{repo}", web::get().to(get_repo))
                    .route("/orgs/{org}/repos", web::get().to(get_org_repos_filtered))
                    .route("/search/repos", web::get().to(search_repositories_handler))
                    .route("/users/{username}/starred_repos", web::get().to(list_starred_repos_handler))
                    .route("/users/{username}/forked_repos", web::get().to(list_user_forked_repos_handler)),
            )
            .service(
                web::scope("/api/mcp")
                    .route("/index_code/{owner}/{repo}/{path:.*}", web::post().to(index_code_handler))
                    .route("/code/{owner}/{repo}/{path:.*}", web::get().to(get_indexed_code_handler))
                    .route("/indexed_code_metadata", web::get().to(list_indexed_code_metadata_handler)),
            )
    })
    .bind(listen_address)?
    .run()
    .await?;

    Ok(())
}
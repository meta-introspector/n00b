// Standard library imports
use std::sync::Arc;
use std::path::PathBuf;

// External crate imports
use anyhow::Result;
use actix_web::{web, App}; // Explicitly import App, web
use actix_web::test; // Explicitly import test module
use actix_web::web::Bytes; // Explicitly import Bytes
use tempfile::tempdir;

// Crate-level imports
use crate::mock_github_client::MockGitHubClient;
use crate::code_indexer::CodeIndexer;
use crate::github_cache::CachedGitHubClient; // Used in this test

// Internal module imports (relative to parent 'server' module)
use super::super::state::AppState;
use super::super::handlers::{
    index_code_handler
};

#[actix_web::test]
async fn test_index_code_fetch_error() -> Result<()> {
    let mock_gh_client = Arc::new(MockGitHubClient::new()); // No content added
    let owner = "testowner";
    let repo = "testrepo";
    let file_path = "src/nonexistent.rs";

    let dir = tempdir()?;
    let cache_path = dir.path().to_path_buf();
    let cached_gh_client_mock = Arc::new(crate::github_cache::CachedGitHubClient::new(mock_gh_client, cache_path)?);
    let code_indexer_path = tempdir()?;
    let code_indexer = Arc::new(CodeIndexer::new(code_indexer_path.path().to_path_buf())?);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { github_client: cached_gh_client_mock.clone(), code_indexer }))
            .service(
                web::scope("/api/mcp")
                    .route("/index_code/{owner}/{repo}/{path:.*}", web::post().to(index_code_handler))
            ),
    )
    .await;

    let index_req = test::TestRequest::post().uri(&format!("/api/mcp/index_code/{}/{}/{}", owner, repo, file_path)).to_request();
    let index_resp = test::call_service(&app, index_req).await;
    assert_eq!(index_resp.status(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    let response_body = test::read_body(index_resp).await;
    assert!(response_body.starts_with(&Bytes::from("Failed to fetch code content: Mock repo content not found")));

    Ok(())
}

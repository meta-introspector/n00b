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
    get_indexed_code_handler
};

#[actix_web::test]
async fn test_get_indexed_code_not_found() -> Result<()> {
    let mock_gh_client = Arc::new(MockGitHubClient::new());
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
                    .route("/code/{owner}/{repo}/{path:.*}", web::get().to(get_indexed_code_handler))
            ),
    )
    .await;

    let get_req = test::TestRequest::get().uri(&format!("/api/mcp/code/{}/{}/{}", owner, repo, file_path)).to_request();
    let get_resp = test::call_service(&app, get_req).await;
    assert_eq!(get_resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    let response_body = test::read_body(get_resp).await;
    assert!(response_body.starts_with(&Bytes::from(format!("Code file {}/{}/{} not found in index", owner, repo, file_path))));

    Ok(())
}

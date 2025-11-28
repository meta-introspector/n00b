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
use crate::code_indexer::{CodeIndexer, CodeFileMetadata}; // Import CodeFileMetadata here
use crate::github_cache::CachedGitHubClient; // Used in this test

// Internal module imports (relative to parent 'server' module)
use super::super::state::AppState;
use super::super::handlers::{
    index_code_handler, get_indexed_code_handler, list_indexed_code_metadata_handler
};

#[actix_web::test]
async fn test_index_and_retrieve_code_success() -> Result<()> {
    let mock_gh_client = Arc::new(MockGitHubClient::new());
    let owner = "testowner";
    let repo = "testrepo";
    let file_path = "src/main.rs";
    let file_content = "fn main() { println!(\"Hello, mcp server!\"); }";
    mock_gh_client.add_repo_content(owner.to_string(), repo.to_string(), file_path.to_string(), file_content.to_string());

    let dir = tempdir()?;
    let cache_path = dir.path().to_path_buf();
    let cached_gh_client_mock = Arc::new(crate::github_cache::CachedGitHubClient::new(mock_gh_client, cache_path)?);
    let code_indexer_path = tempdir()?;
    let code_indexer = Arc::new(CodeIndexer::new(code_indexer_path.path().to_path_buf())?);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { github_client: cached_gh_client_mock.clone(), code_indexer: code_indexer.clone() }))
            .service(
                web::scope("/api/mcp")
                    .route("/index_code/{owner}/{repo}/{path:.*}", web::post().to(index_code_handler))
                    .route("/code/{owner}/{repo}/{path:.*}", web::get().to(get_indexed_code_handler))
                    .route("/indexed_code_metadata", web::get().to(list_indexed_code_metadata_handler))
            ),
    )
    .await;

    // Test indexing code
    let index_req = test::TestRequest::post().uri(&format!("/api/mcp/index_code/{}/{}/{}", owner, repo, file_path)).to_request();
    let index_resp = test::call_service(&app, index_req).await;
    assert!(index_resp.status().is_success());
    let indexed_metadata: crate::code_indexer::CodeFileMetadata = test::read_body_json(index_resp).await;
    assert_eq!(indexed_metadata.repo_full_name, format!( "{}/{}", owner, repo));
    assert_eq!(indexed_metadata.file_path, file_path);
    assert!(indexed_metadata.keywords_found.contains(&"mcp".to_string()));


    // Test retrieving indexed code
    let get_req = test::TestRequest::get().uri(&format!("/api/mcp/code/{}/{}/{}", owner, repo, file_path)).to_request();
    let get_resp = test::call_service(&app, get_req).await;
    assert!(get_resp.status().is_success());
    let retrieved_content = test::read_body(get_resp).await;
    assert_eq!(retrieved_content, Bytes::from(file_content.to_string()));

    // Test listing metadata
    let list_req = test::TestRequest::get().uri("/api/mcp/indexed_code_metadata").to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert!(list_resp.status().is_success());
    let metadata_list: Vec<crate::code_indexer::CodeFileMetadata> = test::read_body_json(list_resp).await;
    assert_eq!(metadata_list.len(), 1);
    assert_eq!(metadata_list[0].file_path, file_path);

    Ok(())
}

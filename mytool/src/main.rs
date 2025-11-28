use std::env;
use std::sync::Arc;
use std::path::PathBuf;

mod github;
mod cache;
mod github_cache;
mod server;
mod mock_github_client; // Add the new mock_github_client module
mod code_indexer;
mod syscall_types;
mod syscall_executor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Get GitHub Token from environment variable
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable must be set");

    // Initialize the live GitHub client
    let live_github_client = Arc::new(github::LiveGitHubClient::new(github_token));

    // Define cache path
    let mut cache_path = PathBuf::from(env::temp_dir());
    cache_path.push("mytool_rocksdb_cache");

    // Initialize the cached GitHub client
    let cached_github_client = Arc::new(github_cache::CachedGitHubClient::new(live_github_client, cache_path.clone())?);

    // Initialize the CodeIndexer
    let code_indexer_path = {
        let mut path = PathBuf::from(env::temp_dir());
        path.push("mytool_code_indexer_rocksdb");
        path
    };
    let code_indexer = Arc::new(code_indexer::CodeIndexer::new(code_indexer_path)?);

    // Define the server listen address
    let listen_address = "127.0.0.1:8080".to_string();

    // Run the Actix-web server
    server::run_server(cached_github_client, code_indexer, listen_address).await?;

    Ok(())
}
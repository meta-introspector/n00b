mod cli;
mod github;
mod repo_index;
mod trust_metric;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    cli::run_cli().await
}
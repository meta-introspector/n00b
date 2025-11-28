use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::Result;

use crate::github_cache::CachedGitHubClient;
use crate::github::{GitHubClient, filter_repos, RepoMetadata};
use crate::code_indexer::CodeIndexer; // Import CodeIndexer

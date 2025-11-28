//! # Prelude
//!
//! This module re-exports commonly used traits and types within the `server` module
//! to simplify imports and improve code readability.
//!
//! By using `use super::prelude::*;`, modules can quickly bring into scope
//! essential components for building Actix-web services, interacting with GitHub,
//! and managing application state.

pub use actix_web::{web, App, HttpServer, Responder, HttpResponse};
pub use serde::{Deserialize, Serialize};
pub use std::sync::Arc;
pub use anyhow::Result;

pub use crate::github_cache::CachedGitHubClient;
pub use crate::github::{GitHubClient, filter_repos, RepoMetadata};
pub use crate::code_indexer::CodeIndexer;

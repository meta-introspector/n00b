//! # Prelude
//!
//! This module re-exports commonly used traits and types within the `server` module
//! to simplify imports and improve code readability.
//!
//! By using `use super::prelude::*;`, modules can quickly bring into scope
//! essential components for building Actix-web services, interacting with GitHub,
//! and managing application state.

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::Result;

use crate::github_cache::CachedGitHubClient;
use crate::github::{GitHubClient, filter_repos, RepoMetadata};
use crate::code_indexer::CodeIndexer; // Import CodeIndexer

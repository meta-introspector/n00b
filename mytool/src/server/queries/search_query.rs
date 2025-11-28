use super::super::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    q: String,
}
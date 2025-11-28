use super::super::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterQuery {
    query: Option<String>,
}
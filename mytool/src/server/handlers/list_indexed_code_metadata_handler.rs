use super::super::prelude::*;
use super::super::state::AppState;
use crate::code_indexer::CodeFileMetadata; // Import CodeFileMetadata directly as it's a return type
use actix_web::web; // Explicitly import web
use log::error;

/// Handles requests to list all indexed code file metadata.
///
/// This endpoint retrieves a list of `CodeFileMetadata` for all code files
/// that have been indexed by the `CodeIndexer`. This provides an overview
/// of the code assets managed by the system.
///
/// # Arguments
/// * `data` - A `web::Data` extractor providing access to the shared `AppState`,
///            including the `CodeIndexer`.
///
/// # Returns
/// A `Result` which resolves to an `HttpResponse`.
/// - On success, returns `HttpResponse::Ok()` with a JSON array of `CodeFileMetadata`.
/// - On failure (e.g., database error, deserialization error), returns
///   `HttpResponse::InternalServerError()` with an error message.
pub async fn list_indexed_code_metadata_handler(
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    match data.code_indexer.list_indexed_code_metadata() {
        Ok(metadata_list) => Ok(HttpResponse::Ok().json(metadata_list)),
        Err(e) => {
            log::error!("Error listing indexed code metadata: {:?}", e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to list indexed code metadata: {}", e)))
        }
    }
}

use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::get_code_path::GetCodePath; // Explicit module import for path struct
use actix_web::web; // Explicitly import web
use log::error;

/// Handles requests to retrieve the content of an indexed code file.
///
/// This endpoint fetches the content of a code file that has previously been
/// indexed by the `CodeIndexer`. It retrieves the raw content from the
/// persistent store.
///
/// # Arguments
/// * `path` - A `web::Path` extractor containing the `GetCodePath` struct, which
///            deserializes the `{owner}`, `{repo}`, and `{path}` from the URL path.
/// * `data` - A `web::Data` extractor providing access to the shared `AppState`,
///            including the `CodeIndexer`.
///
/// # Returns
/// A `Result` which resolves to an `HttpResponse`.
/// - On success, returns `HttpResponse::Ok()` with the raw file content as the response body.
/// - If the file is not found in the index, returns `HttpResponse::NotFound()` with an
///   appropriate error message.
/// - On internal errors (e.g., database access issues), returns
///   `HttpResponse::InternalServerError()` with an error message.
pub async fn get_indexed_code_handler(
    path: web::Path<GetCodePath>,
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let owner = path.owner.as_str();
    let repo = path.repo.as_str();
    let file_path = path.file_path.as_str();

    match data.code_indexer.get_code_file_content(owner, repo, file_path) {
        Ok(Some(content)) => Ok(HttpResponse::Ok().body(content)),
        Ok(None) => Ok(HttpResponse::NotFound().body(format!("Code file {}/{}/{} not found in index", owner, repo, file_path))),
        Err(e) => {
            log::error!("Error retrieving indexed code file {}/{}/{}: {:?}", owner, repo, file_path, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to retrieve indexed code file: {}", e)))
        }
    }
}

use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::GetCodePath; // This handler doesn't use GetCodePath, but keeping the import structure for now.
use crate::code_indexer::CodeFileMetadata; // Import CodeFileMetadata directly as it's a return type

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

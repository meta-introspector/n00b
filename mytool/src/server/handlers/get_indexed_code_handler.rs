use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::GetCodePath;

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

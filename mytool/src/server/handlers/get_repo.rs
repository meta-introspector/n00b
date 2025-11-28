use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::RepoPath;

pub async fn get_repo(path: web::Path<RepoPath>, data: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    let owner = path.owner.as_str();
    let repo_name = path.repo.as_str();
    match data.github_client.get_repo_info(owner, repo_name).await {
        Ok(repo_info) => Ok(HttpResponse::Ok().json(repo_info)),
        Err(e) => {
            log::error!("Error fetching repo {}/{}: {:?}", owner, repo_name, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to fetch repo {}/{}: {}", owner, repo_name, e)))
        }
    }
}

use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::UserReposPath;

pub async fn list_starred_repos_handler(
    path: web::Path<UserReposPath>,
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let username = path.username.as_str();
    match data.github_client.list_starred_repos(username).await {
        Ok(repos) => Ok(HttpResponse::Ok().json(repos)),
        Err(e) => {
            log::error!("Error listing starred repositories for user '{}': {:?}", username, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to list starred repositories: {}", e)))
        }
    }
}

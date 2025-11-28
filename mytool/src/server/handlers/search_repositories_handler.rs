use super::super::prelude::*;
use super::super::state::AppState;
use super::super::queries::SearchQuery;

pub async fn search_repositories_handler(
    query: web::Query<SearchQuery>,
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let q = &query.q;
    match data.github_client.search_repositories(q).await {
        Ok(results) => Ok(HttpResponse::Ok().json(results.items)), // Return only items for simplicity
        Err(e) => {
            log::error!("Error searching repositories with query '{}': {:?}", q, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to search repositories: {}", e)))
        }
    }
}

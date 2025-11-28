use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::OrgReposPath;
use super::super::queries::FilterQuery;
use crate::github::filter_repos; // filter_repos is not in prelude

pub async fn get_org_repos_filtered(
    path: web::Path<OrgReposPath>,
    query: web::Query<FilterQuery>,
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let org = path.org.as_str();
    match data.github_client.list_org_repos(org).await {
        Ok(repos) => {
            if let Some(q) = &query.query {
                let filters: Vec<&str> = q.split_whitespace().collect();
                let filtered_repos = filter_repos(repos, &filters);
                Ok(HttpResponse::Ok().json(filtered_repos))
            } else {
                Ok(HttpResponse::Ok().json(repos))
            }
        }
        Err(e) => {
            log::error!("Error fetching org {} repositories: {:?}", org, e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to fetch org {} repositories: {}", org, e)))
        }
    }
}

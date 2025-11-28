use super::super::prelude::*;
use super::super::state::AppState;
use super::super::paths::UserPath;

pub async fn get_user(path: web::Path<UserPath>, data: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    let username = path.username.as_str();
    match data.github_client.get_user_info(username).await {
        Ok(user_info) => Ok(HttpResponse::Ok().json(user_info)),
        Err(e) => {
            log::error!("Error fetching user {}: {:?}", username, e);
            // In a real application, you might want to return different status codes based on the error type
            // e.g., 404 for not found, 500 for internal server error
            Ok(HttpResponse::InternalServerError().body(format!("Failed to fetch user {}: {}", username, e)))
        }
    }
}

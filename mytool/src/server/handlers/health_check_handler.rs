use super::super::prelude::*;

/// Handles requests to the health check endpoint.
///
/// This endpoint simply returns an "OK" response to indicate that the server is running.
/// It does not require any path or query parameters and does not access application state.
///
/// # Returns
/// An `HttpResponse::Ok()` with the body "OK".
pub async fn health_check() -> Result<impl Responder, actix_web::Error> {
    Ok(HttpResponse::Ok().body("OK"))
}

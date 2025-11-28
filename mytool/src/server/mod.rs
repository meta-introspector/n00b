use actix_web::App;

pub mod prelude;
pub mod state;
pub mod paths;
pub mod queries;
pub mod handlers;
pub mod runner;

// Conditional compilation for tests module
#[cfg(test)]
pub mod tests;
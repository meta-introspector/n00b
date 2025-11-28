use actix_web::App;

pub mod prelude;
pub mod state;
pub mod paths;
pub mod queries;
pub mod handlers;
pub mod runner;
pub use runner::run_server;

// Conditional compilation for tests module
#[cfg(test)]
pub mod tests;